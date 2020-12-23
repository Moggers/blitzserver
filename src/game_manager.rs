use crate::twoh::TwoH;

use super::diesel::prelude::*;
use super::models::{
    File, Game, GameMod, Map, Mod, NewFile, NewNation, NewPlayer, NewPlayerTurn, NewTurn, Player,
    PlayerTurn, Turn,
};
use crossbeam_channel::{Receiver, Sender};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use notify::Watcher;
use std::io::Write;

pub struct GameManager {
    db_pool: r2d2::Pool<ConnectionManager<PgConnection>>,
    tmp_dir: std::path::PathBuf,
    dom5_bin: std::path::PathBuf,
    port_range: [i32; 2],
    sender: Sender<ManagerMsg>,
    receiver: Receiver<ManagerMsg>,
    proc_senders: std::collections::HashMap<i32, Sender<GameCmd>>,
}

pub struct GameMsg {
    pub id: i32,
    pub cmd: GameCmd,
}

pub enum GameCmd {
    LaunchCmd(LaunchCmd),
    SetTimerCmd,
    RebootCmd,
}

pub struct LaunchCmd {
    pub countdown: std::time::Duration,
}

pub enum ManagerMsg {
    Start(i32),
    Stop(i32),
    GameMsg(GameMsg),
    GameDied(i32), // Odd sheep, this one comes from the servers and tells the monitor it's dead
}

pub struct GameManagerConfig<'a> {
    pub db_pool: &'a r2d2::Pool<ConnectionManager<PgConnection>>,
    pub tmp_dir: &'a std::path::Path,
    pub dom5_bin: &'a std::path::Path,
    pub port_range: &'a [i32; 2],
}

struct Dom5Proc {
    pub name: String,
    pub port: i32,
    pub era: i32,
    pub mapname: String,
    pub datadir: String,
    pub savedir: std::path::PathBuf,
    pub game_id: i32,
    db_pool: r2d2::Pool<ConnectionManager<PgConnection>>,
}

impl Dom5Proc {
    fn add_string_to_domcmd<'a>(&self, cmd: &'a str) {
        let mut file = if self.savedir.join("cmd").exists() {
            std::fs::File::open(self.savedir.join("domcmd")).unwrap()
        } else {
            std::fs::File::create(self.savedir.join("domcmd")).unwrap()
        };
        write!(file, "{}", cmd).unwrap();
    }
    fn set_countdown(&self, countdown: u64) {
        self.add_string_to_domcmd(&format!("settimeleft {}", countdown));
    }

    fn set_timer(&mut self) {
        let db = self.db_pool.get().unwrap();
        let game: Game = crate::schema::games::dsl::games
            .filter(crate::schema::games::dsl::id.eq(self.game_id))
            .get_result(&db)
            .unwrap();
        if let Some(timer) = game.timer {
            self.add_string_to_domcmd(&format!("setinterval {}", timer));
        }
    }

    fn update_nations(&self, bin: &std::path::Path) {
        let db = self.db_pool.get().unwrap();
        let res = String::from_utf8(
            std::process::Command::new(bin)
                .env("DOM5_CONF", &self.datadir)
                .arg("--listnations")
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap();
        let re = regex::Regex::new(r#"^ *([0-9]+) +([^,]+), (.+)"#).unwrap();
        let mut new_nations: Vec<NewNation> = vec![];
        for line in res.lines() {
            match re.captures(line) {
                Some(s) => {
                    new_nations.push(NewNation {
                        game_id: self.game_id,
                        nation_id: s.get(1).unwrap().as_str().parse().unwrap(),
                        name: s.get(2).unwrap().as_str().to_owned(),
                        epithet: s.get(3).unwrap().as_str().to_owned(),
                    });
                }
                None => {}
            }
        }
        use super::schema::nations::*;
        use diesel::pg::upsert::excluded;
        diesel::insert_into(super::schema::nations::table)
            .values(new_nations)
            .on_conflict((game_id, nation_id))
            .do_update()
            .set((name.eq(excluded(name)), epithet.eq(excluded(epithet))))
            .execute(&db)
            .unwrap();
    }
    fn handle_statusdump_update(&mut self) {
        let db = self.db_pool.get().unwrap();
        let status_dump = match std::fs::File::open(self.savedir.join("statusdump.txt")) {
            Err(_) => {
                return;
            }
            Ok(f) => super::statusdump::StatusDump::from_file(f),
        };
        if status_dump.turn > 0 {
            let ftherlnd = TwoH::read_file(&self.savedir.join("ftherlnd"));
            let new_file: File = NewFile::new("ftherlnd", &ftherlnd.file_contents).insert(&db);
            use super::schema::turns::dsl::*;
            diesel::insert_into(turns)
                .values(NewTurn {
                    game_id: self.game_id,
                    turn_number: status_dump.turn,
                    file_id: new_file.id,
                })
                .on_conflict((game_id, turn_number))
                .do_update()
                .set(file_id.eq(new_file.id))
                .execute(&db)
                .unwrap();
        }
    }
    fn populate_savegame(&self) {
        std::fs::create_dir_all(&self.savedir).unwrap();
        let db = self.db_pool.get().unwrap();
        let latest_turn: Vec<(Turn, File)> = {
            use super::schema::turns::dsl::*;
            turns
                .filter(game_id.eq(self.game_id))
                .order(turn_number.desc())
                .inner_join(
                    super::schema::files::dsl::files.on(super::schema::files::dsl::id.eq(file_id)),
                )
                .limit(1)
                .get_results(&db)
                .unwrap()
        };
        if latest_turn.len() == 0 {
            let pf: Vec<(Player, File)> = {
                use super::schema::players::dsl::*;
                players
                    .filter(game_id.eq(self.game_id))
                    .inner_join(
                        super::schema::files::dsl::files
                            .on(super::schema::files::dsl::id.eq(file_id)),
                    )
                    .get_results(&db)
                    .unwrap()
            };

            for (_player, file) in pf.iter() {
                let mut os_file =
                    std::fs::File::create(&self.savedir.join(&file.filename)).unwrap();
                os_file.write_all(&file.filebinary).unwrap();
            }
        } else {
            for (turn, file) in latest_turn.iter() {
                let mut os_file =
                    std::fs::File::create(&self.savedir.join(&file.filename)).unwrap();
                os_file.write_all(&file.filebinary).unwrap();
                use super::schema::player_turns::dsl::*;
                for (player_turn, trnfile) in player_turns
                    .filter(
                        game_id
                            .eq(self.game_id)
                            .and(turn_number.eq(turn.turn_number)),
                    )
                    .inner_join(
                        super::schema::files::dsl::files
                            .on(super::schema::files::dsl::id.eq(trnfile_id)),
                    )
                    .get_results::<(PlayerTurn, File)>(&db)
                    .unwrap()
                    .iter()
                {
                    let mut os_file =
                        std::fs::File::create(&self.savedir.join(&trnfile.filename)).unwrap();
                    os_file.write_all(&trnfile.filebinary).unwrap();
                    if let Some(twohid) = player_turn.twohfile_id {
                        let twohfile: File = super::schema::files::dsl::files
                            .filter(super::schema::files::dsl::id.eq(twohid))
                            .get_result(&db)
                            .unwrap();
                        let mut os_file =
                            std::fs::File::create(&self.savedir.join(&twohfile.filename)).unwrap();
                        os_file.write_all(&twohfile.filebinary).unwrap();
                    }
                }
            }
        }
    }
    fn populate_mods(&mut self) {
        let db = self.db_pool.get().unwrap();
        let mods: Vec<(GameMod, Mod, File)> = crate::schema::game_mods::dsl::game_mods
            .filter(crate::schema::game_mods::dsl::game_id.eq(self.game_id))
            .inner_join(
                crate::schema::mods::dsl::mods
                    .on(crate::schema::mods::dsl::id.eq(crate::schema::game_mods::dsl::mod_id)),
            )
            .inner_join(
                crate::schema::files::dsl::files
                    .on(crate::schema::mods::dsl::file_id.eq(crate::schema::files::dsl::id)),
            )
            .get_results::<(GameMod, Mod, File)>(&db)
            .unwrap();
        let mod_dir = std::path::PathBuf::from(&self.datadir).join("mods");
        if !mod_dir.exists() {
            std::fs::create_dir_all(&mod_dir).unwrap();
        }
        for (_, _, cmodfile) in mods.iter() {
            let mut archive =
                zip::ZipArchive::new(std::io::Cursor::new(&cmodfile.filebinary)).unwrap();
            for i in 0..archive.len() {
                let mut f = archive.by_index(i).unwrap();
                if f.is_dir() {
                    if !mod_dir.join(f.name()).exists() {
                        std::fs::create_dir_all(&mod_dir.join(f.name())).unwrap();
                    }
                } else {
                    let mut os_f = std::fs::File::create(&mod_dir.join(f.name())).unwrap();
                    std::io::copy(&mut f, &mut os_f).unwrap();
                }
            }
        }
    }

    fn handle_2h_update(&mut self, path: &std::path::PathBuf) {
        let db = self.db_pool.get().unwrap();
        let twoh = TwoH::read_file(&path);
        let file: File = NewFile::new(
            path.file_name().unwrap().to_str().unwrap(),
            &twoh.file_contents,
        )
        .insert(&db);
        if twoh.turnnumber == 0 {
            use super::schema::players::dsl::*;
            diesel::insert_into(super::schema::players::table)
                .values(&NewPlayer {
                    file_id: file.id,
                    nationid: twoh.nationid,
                    game_id: self.game_id,
                })
                .on_conflict((game_id, nationid))
                .do_update()
                .set(file_id.eq(file.id))
                .execute(&db)
                .unwrap();
        } else {
            use super::schema::player_turns::dsl::*;
            diesel::update(
                player_turns.filter(
                    nation_id
                        .eq(twoh.nationid)
                        .and(game_id.eq(self.game_id))
                        .and(turn_number.eq(twoh.turnnumber)),
                ),
            )
            .set(twohfile_id.eq(file.id))
            .execute(&db)
            .unwrap();
        }
    }

    fn handle_trn_update(&mut self, path: &std::path::PathBuf) {
        let db = self.db_pool.get().unwrap();
        let trn = TwoH::read_file(&path);
        let file: File = NewFile::new(
            path.file_name().unwrap().to_str().unwrap(),
            &trn.file_contents,
        )
        .insert(&db);

        use super::schema::player_turns::dsl::*;
        diesel::insert_into(super::schema::player_turns::table)
            .values(&NewPlayerTurn {
                trnfile_id: file.id,
                nation_id: trn.nationid,
                game_id: self.game_id,
                turn_number: trn.turnnumber,
            })
            .on_conflict((game_id, turn_number, nation_id))
            .do_update()
            .set(trnfile_id.eq(file.id))
            .execute(&db)
            .unwrap();
    }

    fn launch(
        mut self,
        bin: &std::path::Path,
        dc_sender: crossbeam_channel::Sender<ManagerMsg>,
    ) -> Sender<GameCmd> {
        self.update_nations(bin);
        self.populate_savegame();
        let mut arguments = {
            let db = self.db_pool.get().expect("Unable to connect to database");
            let mods: Vec<(GameMod, Mod)> = crate::schema::game_mods::dsl::game_mods
                .filter(crate::schema::game_mods::dsl::game_id.eq(self.game_id))
                .inner_join(
                    crate::schema::mods::dsl::mods
                        .on(crate::schema::mods::dsl::id.eq(crate::schema::game_mods::dsl::mod_id)),
                )
                .get_results::<(GameMod, Mod)>(&db)
                .unwrap();
            mods.iter()
                .flat_map(|(_, m)| vec![String::from("-M"), m.dm_filename.clone()])
                .collect::<Vec<String>>()
        };
        {
            let db = self.db_pool.get().expect("Unable to connect to database");
            use crate::schema::games::dsl::*;
            let game: Game = games.filter(id.eq(self.game_id)).get_result(&db).unwrap();
            arguments.append(&mut vec![
                "--thrones".to_string(),
                game.thrones_t1.to_string(),
                game.thrones_t2.to_string(),
                game.thrones_t3.to_string(),
                "--requiredap".to_string(),
                game.throne_points_required.to_string(),
                "--research".to_string(),
                game.research_diff.to_string(),
                if game.research_rand {
                    ""
                } else {
                    "--norandres"
                }
                .to_string(),
                "--hofsize".to_string(),
                game.hof_size.to_string(),
                "--globals".to_string(),
                game.global_size.to_string(),
                "--indepstr".to_string(),
                game.indepstr.to_string(),
                "--magicsites".to_string(),
                game.magicsites.to_string(),
                "--eventrarity".to_string(),
                game.eventrarity.to_string(),
                "--richness".to_string(),
                game.richness.to_string(),
                "--resources".to_string(),
                game.resources.to_string(),
                "--supplies".to_string(),
                game.supplies.to_string(),
                "--startprov".to_string(),
                game.startprov.to_string(),
                if game.renaming { "--renaming" } else { "" }.to_string(),
                if game.scoregraphs {
                    "--scoregraphs"
                } else {
                    ""
                }
                .to_string(),
                if game.nationinfo {
                    ""
                } else {
                    "--nonationinfo"
                }
                .to_string(),
                "--era".to_string(),
                game.era.to_string(),
                if game.artrest { "" } else { "--noartrest" }.to_string(),
                if game.teamgame { "--teamgame" } else { "" }.to_string(),
                if game.clustered { "--clustered" } else { "" }.to_string(),
                match game.storyevents {
                    0 => "--nostoryevents",
                    1 => "--storyevents",
                    2 => "--allstoryevents",
                    _ => "",
                }
                .to_string(),
                "--newailvl".to_string(),
                game.newailvl.to_string(),
                if game.newai { "" } else { "--nonewai" }.to_string(),
            ]);
        }
        arguments.append(&mut vec![
            String::from("-T"),
            String::from("--tcpserver"),
            String::from("--statusdump"),
            String::from("--mapfile"),
            self.mapname.clone(),
            String::from("--newgame"),
            String::from("--port"),
            format!("{}", self.port),
            format!("{}", self.name),
        ]);
        let mut cmd = std::process::Command::new(bin)
            .env("DOM5_CONF", &self.datadir)
            .args(arguments)
            .spawn()
            .expect(&format!(
                "Failed to launch dom5 binary for game {}",
                self.name
            ));
        self.set_timer();
        let datadir = self.datadir.clone();
        let (file_s, file_r) = crossbeam_channel::unbounded::<notify::Event>();
        let (cmd_s, cmd_r) = crossbeam_channel::unbounded::<GameCmd>();
        std::thread::spawn(move || {
            let mut watcher: notify::RecommendedWatcher =
                notify::Watcher::new_immediate(move |res| match res {
                    Ok(event) => {
                        file_s.send(event).unwrap();
                    }
                    Err(_err) => {}
                })
                .unwrap();
            watcher
                .watch(datadir, notify::RecursiveMode::Recursive)
                .unwrap();
            loop {
                crossbeam_channel::select! {
                    recv(cmd_r) -> res => {
                        match res {
                            Ok(GameCmd::LaunchCmd(cmd)) => {
                                self.set_countdown(cmd.countdown.as_secs());
                            },
                            Ok(GameCmd::SetTimerCmd) => {
                                self.set_timer();
                            }
                            Ok(GameCmd::RebootCmd) => {
                                cmd.kill().unwrap();
                                cmd.wait().unwrap();
                                break;
                            }
                            Err(_) => {
                                panic!("Failed to receive command from game manager");
                            }
                        }
                    },
                    recv(file_r) -> res => {
                        match res {
                            Ok(event) => {
                                match event.kind {
                                    notify::EventKind::Modify(_) | notify::EventKind::Create(_) => {
                                        for path in event.paths {
                                            if path.file_name() == Some(std::ffi::OsStr::new("statusdump.txt")) {
                                                self.handle_statusdump_update();
                                            }
                                            if path.extension() == Some(std::ffi::OsStr::new("trn")) {
                                                self.handle_trn_update(&path);
                                            }
                                            if path.extension() == Some(std::ffi::OsStr::new("2h")) {
                                                self.handle_2h_update(&path);
                                          }
                                        }
                                    },
                                    _ => (),
                                }
                            },
                        _ => panic!("Received error in file watch")
                        }
                    }
                };
            }
            dc_sender.send(ManagerMsg::GameDied(self.game_id)).unwrap();
        });
        cmd_s
    }

    pub fn populate_maps(&mut self) {
        std::fs::create_dir_all(&std::path::PathBuf::from(&self.datadir).join("maps")).unwrap();
        let db = self.db_pool.get().expect("Unable to connect to database");
        let (_, map): (Game, Map) = {
            use super::schema::maps::dsl::*;
            super::schema::games::dsl::games
                .filter(super::schema::games::dsl::id.eq(self.game_id))
                .inner_join(maps.on(id.eq(super::schema::games::dsl::map_id)))
                .get_result(&db)
                .unwrap()
        };
        let (mapfile, tgafile, winterfile) = {
            use super::schema::files::dsl::*;
            (
                files
                    .filter(id.eq(map.mapfile_id))
                    .get_result::<File>(&db)
                    .unwrap(),
                files
                    .filter(id.eq(map.tgafile_id))
                    .get_result::<File>(&db)
                    .unwrap(),
                files
                    .filter(id.eq(map.winterfile_id))
                    .get_result::<File>(&db)
                    .unwrap(),
            )
        };
        std::fs::write(
            std::path::PathBuf::from(&self.datadir)
                .join("maps")
                .join(&mapfile.filename),
            mapfile.filebinary,
        )
        .unwrap();
        std::fs::write(
            std::path::PathBuf::from(&self.datadir)
                .join("maps")
                .join(&tgafile.filename),
            tgafile.filebinary,
        )
        .unwrap();
        std::fs::write(
            std::path::PathBuf::from(&self.datadir)
                .join("maps")
                .join(&winterfile.filename),
            winterfile.filebinary,
        )
        .unwrap();
    }
}

impl GameManager {
    pub fn new(cfg: &GameManagerConfig) -> GameManager {
        let (s, r) = crossbeam_channel::unbounded();
        GameManager {
            db_pool: cfg.db_pool.clone(),
            tmp_dir: cfg.tmp_dir.to_owned(),
            dom5_bin: cfg.dom5_bin.to_owned(),
            port_range: cfg.port_range.clone(),
            sender: s,
            receiver: r,
            proc_senders: std::collections::HashMap::new(),
        }
    }

    pub fn get_sender(&self) -> Sender<ManagerMsg> {
        self.sender.clone()
    }

    pub fn start(&mut self) {
        if !std::path::Path::exists(&self.tmp_dir) {
            std::fs::create_dir(&self.tmp_dir).unwrap();
        }
        if !std::path::Path::exists(&self.tmp_dir.join("games")) {
            std::fs::create_dir(&self.tmp_dir.join("games")).unwrap();
        }
        self.launch_games()
    }

    pub fn monitor(&mut self) {
        for msg in self.receiver.clone().iter() {
            match msg {
                ManagerMsg::GameDied(id) => {
                    let sender = self.launch_game(id);
                    self.proc_senders.insert(id, sender);
                }
                ManagerMsg::Start(id) => {
                    let sender = self.launch_game(id);
                    self.proc_senders.insert(id, sender);
                }
                ManagerMsg::GameMsg(game_cmd) => match self.proc_senders.get(&game_cmd.id) {
                    Some(s) => {
                        if let Err(_) = s.send(game_cmd.cmd) {
                            println!("WARN!!!! Failed to send message to slave for game {}, assumed the server no longer exists and rebooting!", game_cmd.id);
                            let sender = self.launch_game(game_cmd.id);
                            self.proc_senders.insert(game_cmd.id, sender);
                        }
                    }
                    None => {
                        panic!("Tried to control game before it has started");
                    }
                },
                _ => {}
            }
        }
    }

    fn launch_game(&mut self, launch_id: i32) -> Sender<GameCmd> {
        let db = self.db_pool.get().expect("Unable to connect to database");
        let mut game: Game = {
            use super::schema::games::dsl::*;
            games
                .filter(id.eq(launch_id))
                .get_result(&db)
                .expect(&format!("Faled to find game {} to launch", launch_id))
        };
        let tmp_game_path = self.tmp_dir.join("games").join(&game.id.to_string());
        match std::fs::remove_dir_all(&tmp_game_path) {
            Ok(_) => (),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                () // File might not exist..
            }
            Err(e) => println!("Found something odd {:?}", e),
        }
        std::fs::create_dir_all(&tmp_game_path.join("maps")).unwrap();
        std::fs::create_dir_all(&tmp_game_path.join("mods")).unwrap();
        let (_map, mapfile) = {
            use super::schema::maps::dsl::*;
            maps.filter(id.eq(game.map_id))
                .inner_join(
                    super::schema::files::dsl::files
                        .on(super::schema::files::dsl::id.eq(super::schema::maps::dsl::mapfile_id)),
                )
                .get_result::<(Map, File)>(&db)
                .unwrap()
        };
        if let None = game.port {
            let new_port: i32 = diesel::dsl::sql::<diesel::sql_types::Int4>(&format!(
                    "SELECT * FROM generate_series({}, {}) num LEFT OUTER JOIN games g ON g.port=num WHERE g.id IS NULL",
                    self.port_range[0], self.port_range[1]
                ))
                .get_result(&db)
                .unwrap();
            use super::schema::games::dsl::*;
            game = diesel::update(games.filter(id.eq(game.id)))
                .set(port.eq(new_port))
                .get_result(&db)
                .unwrap();
        }
        let mut proc = Dom5Proc {
            game_id: game.id,
            savedir: std::path::PathBuf::from(&tmp_game_path)
                .join("savedgames")
                .join(&game.name),
            name: game.name,
            port: game
                .port
                .expect("No port specified for game, something went wrong!"),
            era: game.era,
            mapname: mapfile.filename.clone(),
            datadir: tmp_game_path.into_os_string().into_string().unwrap(),
            db_pool: self.db_pool.clone(),
        };
        proc.populate_maps();
        proc.populate_mods();
        proc.launch(&self.dom5_bin, self.get_sender())
    }

    fn launch_games(&mut self) {
        let current_games = {
            let db = self.db_pool.get().expect("Unable to connect to database");
            use super::schema::games::dsl::*;
            games.load::<Game>(&db).expect("Error loading games")
        };

        for game in current_games {
            let sender = self.launch_game(game.id);
            self.proc_senders.insert(game.id, sender);
        }
        self.monitor();
    }
}

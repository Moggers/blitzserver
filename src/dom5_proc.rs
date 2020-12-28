use crate::twoh::TwoH;

use super::diesel::prelude::*;
use super::models::{
    File, Game, GameMod, Map, Mod, NewFile, NewNation, NewPlayer, NewPlayerTurn, NewTurn, Player,
    PlayerTurn, Turn,
};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use notify::Watcher;
use std::io::Write;
use std::ops::Add;

pub struct LaunchCmd {
    pub countdown: std::time::Duration,
}

pub enum GameCmd {
    Shutdown,
    LaunchCmd(LaunchCmd),
    SetTimerCmd,
}

pub enum ProcEvent {
    NewTurn,
}

pub struct Dom5ProcHandle {
    pub sender: crossbeam_channel::Sender<GameCmd>,
    pub port: i32,
}

impl Drop for Dom5ProcHandle {
    fn drop(&mut self) {
        self.sender.send(GameCmd::Shutdown).unwrap();
    }
}

pub struct Dom5Proc {
    pub name: String,
    pub era: i32,
    pub mapname: String,
    pub datadir: String,
    pub send_upstream: crossbeam_channel::Sender<ProcEvent>,
    pub savedir: std::path::PathBuf,
    pub game_id: i32,
    pub internal_port_range: [i32; 2],
    pub db_pool: r2d2::Pool<ConnectionManager<PgConnection>>,
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

    fn unset_timer(&self) {
        self.add_string_to_domcmd("setinterval 0");
    }

    fn set_timer(&mut self) {
        let db = self.db_pool.get().unwrap();
        let game: Game = crate::schema::games::dsl::games
            .filter(crate::schema::games::dsl::id.eq(self.game_id))
            .get_result(&db)
            .unwrap();
        if let Some(next_turn) = game.next_turn {
            if next_turn >= std::time::SystemTime::now().add(std::time::Duration::from_secs(60)) {
                self.add_string_to_domcmd(&format!(
                    "setinterval {}",
                    next_turn
                        .duration_since(std::time::SystemTime::now())
                        .unwrap()
                        .as_secs()
                        / 60
                ));
            } else {
                self.add_string_to_domcmd("setinterval 1");
            }
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
    fn handle_new_turn(&self) {
        let db = self.db_pool.get().unwrap();
        let ftherlnd = TwoH::read_file(&self.savedir.join("ftherlnd"));
        let new_file: File = NewFile::new("ftherlnd", &ftherlnd.file_contents).insert(&db);
        use super::schema::turns::dsl::*;
        let existing_turn = turns
            .filter(
                game_id
                    .eq(self.game_id)
                    .and(turn_number.eq(ftherlnd.turnnumber)),
            )
            .get_result::<Turn>(&db);
        if !existing_turn.is_ok() || existing_turn.unwrap().file_id != new_file.id {
            diesel::insert_into(turns)
                .values(NewTurn {
                    game_id: self.game_id,
                    turn_number: ftherlnd.turnnumber,
                    file_id: new_file.id,
                })
                .on_conflict((game_id, turn_number))
                .do_update()
                .set(file_id.eq(new_file.id))
                .execute(&db)
                .unwrap();
            self.unset_timer();
            self.send_upstream.send(ProcEvent::NewTurn).unwrap();
        }
    }
    fn handle_statusdump_update(&mut self) {
        let status_dump = match std::fs::File::open(self.savedir.join("statusdump.txt")) {
            Err(_) => {
                return;
            }
            Ok(f) => super::statusdump::StatusDump::from_file(f),
        };
        if status_dump.turn > 0 {
            self.handle_new_turn();
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
    pub fn populate_mods(&mut self) {
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

    pub fn host_turn(mut self, bin: &std::path::Path) {
        self.update_nations(bin);
        self.populate_savegame();
        self.populate_mods();
        self.populate_maps();
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
            String::from("--mapfile"),
            self.mapname.clone(),
            String::from("-g"),
            format!("{}", self.name),
        ]);
        std::process::Command::new(bin)
            .env("DOM5_CONF", &self.datadir)
            .args(arguments)
            .output()
            .expect(&format!(
                "Failed to launch dom5 binary for game {}",
                self.name
            ));
        self.handle_new_turn();
        for entry in std::fs::read_dir(&self.savedir).unwrap() {
            if let Ok(entry) = entry {
                let file_name = std::path::PathBuf::from(entry.file_name());
                match file_name.extension().and_then(std::ffi::OsStr::to_str) {
                    Some("trn") => {
                        self.handle_trn_update(&self.savedir.join(&file_name.to_path_buf()))
                    }
                    _ => {}
                }
            }
        }
        for entry in std::fs::read_dir(&self.savedir).unwrap() {
            if let Ok(entry) = entry {
                let file_name = std::path::PathBuf::from(entry.file_name());
                match file_name.extension().and_then(std::ffi::OsStr::to_str) {
                    Some("2h") => {
                        self.handle_2h_update(&self.savedir.join(&file_name.to_path_buf()))
                    }
                    _ => {}
                }
            }
        }
        self.send_upstream.send(ProcEvent::NewTurn).unwrap();
    }

    pub fn launch(mut self, bin: &std::path::Path) -> Dom5ProcHandle {
        self.update_nations(bin);
        self.populate_savegame();
        self.populate_mods();
        self.populate_maps();
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
        let new_internal_port = (self.internal_port_range[0]..self.internal_port_range[1])
            .find(|check_port| {
                match std::net::TcpListener::bind(format!("0.0.0.0:{}", check_port)) {
                    Ok(_) => {
                        println!("Bound to {}, using", check_port);
                        true
                    }
                    Err(_) => false,
                }
            })
            .unwrap();
        arguments.append(&mut vec![
            String::from("-T"),
            String::from("--tcpserver"),
            String::from("--statusdump"),
            String::from("--port"),
            format!("{}", new_internal_port),
            String::from("--mapfile"),
            self.mapname.clone(),
            String::from("--newgame"),
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
        let (sender, receiver) = crossbeam_channel::unbounded::<GameCmd>();
        let (file_s, file_r) = crossbeam_channel::unbounded::<notify::Event>();
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
                    recv(receiver) -> res => {
                        match res {
                            Ok(GameCmd::Shutdown) => {
                                cmd.kill().unwrap();
                                cmd.wait().unwrap();
                                break;
                            },
                            Ok(GameCmd::LaunchCmd(cmd)) => {
                                self.set_countdown(cmd.countdown.as_secs());
                            }
                            Ok(GameCmd::SetTimerCmd) => {
                                self.set_timer();
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
        });
        Dom5ProcHandle {
            sender: sender,
            port: new_internal_port,
        }
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
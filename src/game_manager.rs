use super::diesel::prelude::*;
use super::models::{File, Game, Map};
use crossbeam_channel::{Receiver, Sender};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

pub struct GameManager {
    db_pool: r2d2::Pool<ConnectionManager<PgConnection>>,
    tmp_dir: std::path::PathBuf,
    dom5_bin: std::path::PathBuf,
    port_range: [i32; 2],
    sender: Sender<ManagerMsg>,
    receiver: Receiver<ManagerMsg>,
}

pub enum ManagerMsg {
    Start(i32),
    Stop(i32),
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
}

impl Dom5Proc {
    fn launch(&self, bin: &std::path::Path) {
        std::process::Command::new(bin)
            .env("DOM5_CONF", &self.datadir)
            .args(&[
                "-T",
                "--tcpserver",
                "--mapfile",
                &self.mapname,
                &format!("{}", self.era),
                "--newgame",
                "--port",
                &format!("{}", self.port),
                "--era",
                &format!("{}", self.era),
                &format!("{}", self.name),
            ])
            .spawn()
            .expect(&format!(
                "Failed to launch dom5 binary for game {}",
                self.name
            ));
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
        let db = self.db_pool.get().expect("Unable to connect to database");
        for msg in self.receiver.clone().iter() {
            match msg {
                ManagerMsg::Start(id) => {
                    println!("Need to start a new game.. {}", id);
                    self.launch_game(id)
                }
                _ => {}
            }
        }
    }

    fn launch_game(&mut self, launch_id: i32) {
        let db = self.db_pool.get().expect("Unable to connect to database");
        let mut game: Game = {
            use super::schema::games::dsl::*;
            games
                .filter(id.eq(launch_id))
                .get_result(&db)
                .expect(&format!("Faled to find game {} to launch", launch_id))
        };
        let tmp_game_path = self.tmp_dir.join("games").join(&game.name);
        match std::fs::remove_dir_all(&tmp_game_path) {
            Ok(_) => (),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                () // File might not exist..
            }
            Err(e) => panic!(e),
        }
        std::fs::create_dir(&tmp_game_path).unwrap();
        std::fs::create_dir(&tmp_game_path.join("maps")).unwrap();
        std::fs::create_dir(&tmp_game_path.join("mods")).unwrap();
        std::fs::create_dir(&tmp_game_path.join("savedgames")).unwrap();
        let map = {
            use super::schema::maps::dsl::*;
            maps.filter(id.eq(game.map_id))
                .get_result::<Map>(&db)
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
            tmp_game_path.join("maps").join(&mapfile.filename),
            mapfile.filebinary,
        )
        .unwrap();
        std::fs::write(
            tmp_game_path.join("maps").join(&tgafile.filename),
            tgafile.filebinary,
        )
        .unwrap();
        std::fs::write(
            tmp_game_path.join("maps").join(&winterfile.filename),
            winterfile.filebinary,
        )
        .unwrap();
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
        (Dom5Proc {
            name: game.name,
            port: game
                .port
                .expect("No port specified for game, something went wrong!"),
            era: game.era,
            mapname: mapfile.filename.clone(),
            datadir: tmp_game_path.into_os_string().into_string().unwrap()
        })
        .launch(&self.dom5_bin);
    }

    fn launch_games(&mut self) {
        let db = self.db_pool.get().expect("Unable to connect to database");
        let current_games = {
            use super::schema::games::dsl::*;
            games.load::<Game>(&db).expect("Error loading games")
        };

        for game in current_games {
            self.launch_game(game.id);
        }
        self.monitor();
    }
}

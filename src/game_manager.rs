use super::diesel::prelude::*;
use super::models::{File, Game, Map};
use crossbeam_channel::{Receiver, Sender};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

use crate::dom5_proc::{Dom5Proc, Dom5ProcHandle, GameCmd};

pub struct GameManager {
    db_pool: r2d2::Pool<ConnectionManager<PgConnection>>,
    tmp_dir: std::path::PathBuf,
    dom5_bin: std::path::PathBuf,
    port_range: [i32; 2],
    internal_port_range: [i32; 2],
    sender: Sender<ManagerMsg>,
    receiver: Receiver<ManagerMsg>,
    proc_senders: std::collections::HashMap<i32, Sender<GameCmd>>,
}

pub struct GameMsg {
    pub id: i32,
    pub cmd: GameCmd,
}

pub enum ManagerMsg {
    Start(i32),
    Stop(i32),
    GameMsg(GameMsg),
}

pub struct GameManagerConfig<'a> {
    pub db_pool: &'a r2d2::Pool<ConnectionManager<PgConnection>>,
    pub tmp_dir: &'a std::path::Path,
    pub dom5_bin: &'a std::path::Path,
    pub port_range: &'a [i32; 2],
    pub internal_port_range: &'a [i32; 2],
}

/**
 * Manager for servers
 *
 * Responsible for:
 * * receiving messages from frontend and divying them out to the appropriate proxies
 * * Starting new server proxies when requested by the server
 */
impl GameManager {
    pub fn new(cfg: &GameManagerConfig) -> GameManager {
        let (s, r) = crossbeam_channel::unbounded();
        GameManager {
            db_pool: cfg.db_pool.clone(),
            tmp_dir: cfg.tmp_dir.to_owned(),
            dom5_bin: cfg.dom5_bin.to_owned(),
            port_range: cfg.port_range.clone(),
            internal_port_range: cfg.internal_port_range.clone(),
            sender: s,
            receiver: r,
            proc_senders: std::collections::HashMap::new(),
        }
    }

    pub fn get_sender(&self) -> Sender<ManagerMsg> {
        self.sender.clone()
    }

    pub fn start(&mut self) {
        if !std::path::Path::exists(&self.tmp_dir.join("games")) {
            std::fs::create_dir_all(&self.tmp_dir.join("games")).unwrap();
        }
        self.launch_games()
    }

    pub fn monitor(&mut self) {
        loop {
            if let Ok(msg) = self
                .receiver
                .recv_timeout(std::time::Duration::from_secs(5))
            {
                match msg {
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
                        None => {}
                    },
                    _ => {}
                }
            }
            let games_to_roll = {
                let db = self.db_pool.get().unwrap();
                use crate::schema::games::dsl::*;
                games
                    .filter(next_turn.lt(std::time::SystemTime::now()))
                    .get_results::<Game>(&db)
                    .unwrap()
            };
            for game in games_to_roll {
                if let Some(sender) = self.proc_senders.get(&game.id) {
                    sender.send(GameCmd::RollTurn).unwrap();
                }
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
        // Allocate new external port if there is none
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
        let (proxy_s, proxy_r) = crossbeam_channel::unbounded::<GameCmd>();

        // Proxy listener
        let db_pool = self.db_pool.clone();
        let binpath = self.dom5_bin.clone();
        let ipr = self.internal_port_range.clone();
        let mapname = mapfile.filename.clone();

        let base_proc_handle: std::sync::Arc<std::sync::RwLock<std::sync::Weak<Dom5ProcHandle>>> =
            std::sync::Arc::new(std::sync::RwLock::new(std::sync::Weak::new()));
        let proc_handle = base_proc_handle.clone();
        std::thread::spawn(move || {
            let listener =
                std::net::TcpListener::bind(format!("0.0.0.0:{}", game.port.unwrap())).unwrap();
            while let Ok((client_sock, _)) = listener.accept() {
                let our_proc_handle = {
                    let mut locked_handle = proc_handle.write().expect("D5Proc RWLock poisoned");
                    match locked_handle.upgrade() {
                        Some(proc_handle) => proc_handle,
                        None => {
                            let handle = std::sync::Arc::new(
                                Dom5Proc {
                                    game_id: game.id,
                                    savedir: std::path::PathBuf::from(&tmp_game_path.clone())
                                        .join("savedgames")
                                        .join(&game.name),
                                    name: game.name.clone(),
                                    era: game.era,
                                    mapname: mapname.clone(),
                                    datadir: tmp_game_path
                                        .clone()
                                        .into_os_string()
                                        .into_string()
                                        .unwrap(),
                                    db_pool: db_pool.clone(),
                                    internal_port_range: ipr,
                                }
                                .launch(&binpath),
                            );
                            std::thread::sleep(std::time::Duration::from_millis(500)); // Wait for Dom5 to bind its port
                            *locked_handle = std::sync::Arc::downgrade(&handle);
                            handle
                        }
                    }
                };
                std::thread::spawn(move || {
                    let serv_sock =
                        std::net::TcpStream::connect(format!("0.0.0.0:{}", our_proc_handle.port))
                            .unwrap();
                    let mut client_write = client_sock.try_clone().unwrap();
                    let mut serv_write = serv_sock.try_clone().unwrap();
                    let mut client_read = client_sock.try_clone().unwrap();
                    let mut serv_read = serv_sock.try_clone().unwrap();
                    let (exit_s, exit_r) = crossbeam_channel::unbounded::<bool>();
                    let ces = exit_s.clone();
                    let ses = exit_s.clone();
                    let client_handle = std::thread::spawn(move || {
                        match std::io::copy(&mut client_read, &mut serv_write) {
                            _ => {}
                        }
                        ces.send(true).unwrap();
                    });
                    let serv_handle = std::thread::spawn(move || {
                        match std::io::copy(&mut serv_read, &mut client_write) {
                            _ => {}
                        }
                        ses.send(true).unwrap();
                    });
                    exit_r.recv().unwrap();
                    match (
                        serv_sock.shutdown(std::net::Shutdown::Both),
                        client_sock.shutdown(std::net::Shutdown::Both),
                    ) {
                        _ => {}
                    }
                    exit_r.recv().unwrap();
                    serv_handle.join().unwrap();
                    client_handle.join().unwrap();
                    drop(our_proc_handle);
                });
            }
        });

        std::thread::spawn(move || {
            while let Ok(msg) = proxy_r.recv() {
                match msg {
                    _ => match base_proc_handle.read().unwrap().upgrade() {
                        Some(d5ph) => {
                            d5ph.sender.send(msg).unwrap();
                        }
                        None => {}
                    },
                }
            }
        });
        proxy_s
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

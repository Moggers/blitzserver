use super::diesel::prelude::*;
use super::models::Game;
use crossbeam_channel::{Receiver, Sender};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

use crate::dom5_proxy::{Dom5Proxy, Dom5ProxyHandle, GameMsg};

pub struct GameManager {
    db_pool: r2d2::Pool<ConnectionManager<PgConnection>>,
    tmp_dir: std::path::PathBuf,
    dom5_bin: std::path::PathBuf,
    port_range: [i32; 2],
    internal_port_range: [i32; 2],
    sender: Sender<ManagerMsg>,
    receiver: Receiver<ManagerMsg>,
    proxies: std::collections::HashMap<i32, Dom5ProxyHandle>,
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
            proxies: std::collections::HashMap::new(),
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
                        self.launch_game(id);
                    }
                    ManagerMsg::GameMsg(game_cmd) => {
                        let id = game_cmd.id;
                        match self.proxies.get(&id) {
                            Some(ref s) => {
                                if let Err(_) = s.manager_sender.send(game_cmd) {
                                    println!("WARN!!!! Failed to send message to slave for game {}, assumed the server no longer exists and rebooting!", id);
                                    self.launch_game(id);
                                }
                            }
                            None => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    fn launch_game(&mut self, launch_id: i32) {
        let (s, r) = crossbeam_channel::unbounded::<GameMsg>();
        let (is, ir) = crossbeam_channel::bounded::<()>(0);
        let proxy = Dom5Proxy {
            game_id: launch_id,
            db_pool: self.db_pool.clone(),
            game_dir: self.tmp_dir.join("games").join(format!("{}", launch_id)),
            dom5_bin: self.dom5_bin.clone(),
            internal_port_range: self.internal_port_range,
            manager_sender: s,
            manager_receiver: r,
            port_range: self.port_range,
            nextturn_interupt: is,
            nextturn_interupt_recv: ir,
        };
        let proxy_handle = proxy.launch();
        self.proxies.insert(launch_id, proxy_handle);
    }

    fn launch_games(&mut self) {
        let current_games = {
            let db = self.db_pool.get().expect("Unable to connect to database");
            use super::schema::games::dsl::*;
            games.load::<Game>(&db).expect("Error loading games")
        };

        for game in current_games {
            self.launch_game(game.id);
        }
        self.monitor();
    }
}

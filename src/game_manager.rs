use super::diesel::prelude::*;
use super::models::Game;
use crate::msgbus::Msg;
use crossbeam_channel::{Receiver, Sender};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use std::ops::Add;

use crate::dom5_proc::Dom5Proc;

pub struct GameManager {
    db_pool: r2d2::Pool<ConnectionManager<PgConnection>>,
    tmp_dir: std::path::PathBuf,
    dom5_bin: std::path::PathBuf,
    port_range: [i32; 2],
    internal_port_range: [i32; 2],
    sender: Sender<ManagerMsg>,
    receiver: Receiver<ManagerMsg>,
    pub bus_rx: crate::msgbus::MsgBusRx,
    pub bus_tx: crate::msgbus::MsgBusTx,
}

pub enum ManagerMsg {
    Start(i32),
    Archive(i32),
}

pub struct GameManagerConfig<'a> {
    pub bus_rx: crate::msgbus::MsgBusRx,
    pub bus_tx: crate::msgbus::MsgBusTx,
    pub db_pool: &'a r2d2::Pool<ConnectionManager<PgConnection>>,
    pub tmp_dir: &'a std::path::Path,
    pub dom5_bin: &'a std::path::Path,
    pub port_range: &'a [i32; 2],
    pub internal_port_range: &'a [i32; 2],
}

impl GameManager {
    pub fn new(cfg: GameManagerConfig) -> GameManager {
        let (s, r) = crossbeam_channel::unbounded();
        GameManager {
            bus_tx: cfg.bus_tx,
            bus_rx: cfg.bus_rx,
            db_pool: cfg.db_pool.clone(),
            tmp_dir: cfg.tmp_dir.to_owned(),
            dom5_bin: cfg.dom5_bin.to_owned(),
            port_range: cfg.port_range.clone(),
            internal_port_range: cfg.internal_port_range.clone(),
            sender: s,
            receiver: r,
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
    fn kill_game(&mut self, game_id: i32) {}
    pub fn monitor(&mut self) {}

    fn host_turn(
        launch_id: i32,
        db_pool: r2d2::Pool<ConnectionManager<PgConnection>>,
    ) -> std::time::SystemTime {
        let db = db_pool.get().unwrap();
        let game = Game::get(launch_id, &db).unwrap();
        let dom5_proc = Dom5Proc::new(game, db_pool.clone());
        dom5_proc.host_turn();
        log::debug!("Turn for game {} hosted", launch_id);
        let game = Game::get(launch_id, &db).unwrap();
        match game.next_turn {
            None => std::time::SystemTime::now().add(std::time::Duration::from_secs(99_999_999)),
            Some(t) => t,
        }
    }

    fn launch_scheduler(&mut self, launch_id: i32) {
        let bus_rx = self.bus_rx.new_recv();
        let bus_tx = self.bus_tx.clone();
        let db_pool = self.db_pool.clone();
        std::thread::spawn(move || {
            let mut timeout =
                std::time::SystemTime::now().add(std::time::Duration::from_secs(99_999_999));
            let mut game = {
                let db = db_pool.get().unwrap();
                Game::get(launch_id, &db).unwrap()
            };
            if let Some(t) = game.next_turn {
                timeout = t;
            }
            loop {
                match timeout.duration_since(std::time::SystemTime::now()) {
                    Ok(t) => match bus_rx.recv_timeout(t) {
                        Ok(Msg::GameSchedule(schdmsg)) if schdmsg.game_id == launch_id => {
                            timeout = schdmsg.schedule;
                        }
                        Err(_) | _ => {
                            if std::time::SystemTime::now() >= timeout {
                                log::debug!("Game {} scheduled for new turn right now", launch_id);
                                timeout = Self::host_turn(launch_id, db_pool.clone());
                            }
                        }
                    },
                    Err(_) => {
                        log::debug!("Game {} missed scheduled turn roll, running now", launch_id);
                        timeout = Self::host_turn(launch_id, db_pool.clone());
                    }
                }
            }
        });
    }

    fn launch_game(&mut self, launch_id: i32) {
        self.launch_scheduler(launch_id);
        let emu = crate::dom5_emu::Dom5Emu::new(
            launch_id,
            self.bus_tx.clone(),
            self.bus_rx.new_recv(),
            self.db_pool.clone(),
        );
        emu.launch();
    }

    fn launch_games(&mut self) {
        let current_games: Vec<Game> = {
            let db = self.db_pool.get().expect("Unable to connect to database");
            use super::schema::games::dsl as games_dsl;
            games_dsl::games
                .filter(games_dsl::archived.eq(false))
                .get_results(&db)
                .expect("Error loading games")
        };

        for game in current_games {
            self.launch_game(game.id);
            let dom5_proc = Dom5Proc::new(game, self.db_pool.clone());
            dom5_proc.update_nations();
        }
        self.monitor();
    }
}

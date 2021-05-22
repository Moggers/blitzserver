use super::diesel::prelude::*;
use super::models::{DiscordConfig, Game, Turn};
use crate::msgbus::{
    CreateGameMsg, EraChangedMsg, GameScheduleMsg, ModsChangedMsg, Msg, MsgBusTx, NewTurnMsg,
    TurnHostStartMsg,
};
use crossbeam_channel::Sender;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

use crate::dom5_proc::Dom5Proc;

pub struct GameManager {
    db_pool: r2d2::Pool<ConnectionManager<PgConnection>>,
    tmp_dir: std::path::PathBuf,
    sender: Sender<ManagerMsg>,
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
}

impl GameManager {
    pub fn new(cfg: GameManagerConfig) -> GameManager {
        let (s, _r) = crossbeam_channel::unbounded();
        GameManager {
            bus_tx: cfg.bus_tx,
            bus_rx: cfg.bus_rx,
            db_pool: cfg.db_pool.clone(),
            tmp_dir: cfg.tmp_dir.to_owned(),
            sender: s,
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
            let due_games = {
                let db = self.db_pool.get().unwrap();
                Game::get_due_games(&db).unwrap()
            };
            for game in due_games {
                Self::scheduled_host(game.id, self.bus_tx.clone(), self.db_pool.clone());
            }
            let timer = {
                let db = self.db_pool.get().unwrap();
                let now = std::time::SystemTime::now();
                Game::get_next_wakeup(&db)
                    .unwrap_or(None)
                    .unwrap_or(now)
                    .duration_since(now)
                    .unwrap_or(std::time::Duration::from_nanos(1))
            };
            match self.bus_rx.recv_timeout(timer) {
                Ok(Msg::CreateGame(CreateGameMsg { game_id })) => {
                    self.launch_game(game_id);
                }
                Ok(Msg::EraChanged(EraChangedMsg { game_id, .. }))
                | Ok(Msg::ModsChanged(ModsChangedMsg { game_id })) => {
                    let db = self.db_pool.get().unwrap();
                    let game = Game::get(game_id, &db).unwrap();
                    let mut dom5_proc = Dom5Proc::new(game);
                    dom5_proc.populate_mods(&db);
                    dom5_proc.update_nations(&db);
                }
                Ok(Msg::OrdersSubmitted(orders)) => {
                    let db = self.db_pool.get().unwrap();
                    let turn = Turn::get(orders.game_id, &db).unwrap();
                    let player_turns = turn.get_player_turns(&db).unwrap();
                    if player_turns.iter().all(|pt| pt.status == 2) {
                        Self::scheduled_host(
                            orders.game_id,
                            self.bus_tx.clone(),
                            self.db_pool.clone(),
                        );
                    }
                }
                _ => {}
            }
        }
    }

    fn host_turn(
        launch_id: i32,
        sender: &MsgBusTx,
        db_pool: r2d2::Pool<ConnectionManager<PgConnection>>,
    ) -> Option<std::time::SystemTime> {
        let db = db_pool.get().unwrap();
        let game = Game::get(launch_id, &db).unwrap();
        let turn_n = match Turn::get(game.id, &db) {
            Ok(turn) => turn.turn_number,
            Err(_) => 0,
        };
        sender
            .send(Msg::TurnHostStart(TurnHostStartMsg {
                game_id: game.id,
                turn_number: turn_n + 1,
            }))
            .unwrap();
        let dom5_proc = Dom5Proc::new(game);
        dom5_proc.host_turn(db_pool);
        log::debug!("Turn for game {} hosted", launch_id);
        let game = Game::get(launch_id, &db).unwrap();
        let turn = Turn::get(game.id, &db).unwrap();
        sender
            .send(Msg::NewTurn(NewTurnMsg {
                game_id: game.id,
                turn_number: turn.turn_number,
            }))
            .unwrap();
        game.next_turn
    }

    pub fn scheduled_host(
        launch_id: i32,
        bus_tx: crate::msgbus::MsgBusTx,
        db_pool: r2d2::Pool<ConnectionManager<PgConnection>>,
    ) {
        log::info!("Hosting turn for game {}", launch_id);
        let timer = Self::host_turn(launch_id, &bus_tx, db_pool);
        if let Some(timer) = timer {
            bus_tx
                .send(Msg::GameSchedule(GameScheduleMsg {
                    game_id: launch_id,
                    schedule: timer,
                }))
                .unwrap();
        }
    }

    fn launch_game(&mut self, launch_id: i32) {
        let db = self.db_pool.get().unwrap();
        let game = Game::get(launch_id, &db).unwrap();
        let mut dom5_proc = Dom5Proc::new(game);
        dom5_proc.populate_mods(&db);
        dom5_proc.update_nations(&db);
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
        }
        self.monitor();
    }
}

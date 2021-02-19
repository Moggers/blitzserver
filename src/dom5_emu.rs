use crate::diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};
use crate::models::{Game, Map, Nation};
use crate::msgbus::{Msg, PktMsg};
use crate::packets::BodyContents;
use crate::packets::{
    AstralPacketResp, DisconnectResp, GameInfoResp, LoadingMessageResp, MapFileResp, MapResp,
    PAResp, Packet, PasswordsResp, TrnResp, TwoHCrcResp,
};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use std::env;
use std::ops::Add;
pub struct Dom5Emu {
    game_id: i32,
    bus_rx: crate::msgbus::MsgBusRx,
    bus_tx: crate::msgbus::MsgBusTx,
    pub db_pool: r2d2::Pool<ConnectionManager<PgConnection>>,
}

impl Dom5Emu {
    pub fn new(
        game_id: i32,
        bus_tx: crate::msgbus::MsgBusTx,
        bus_rx: crate::msgbus::MsgBusRx,
        db_pool: r2d2::Pool<ConnectionManager<PgConnection>>,
    ) -> Self {
        Self {
            game_id,
            bus_rx,
            bus_tx,
            db_pool,
        }
    }

    fn generate_gameinfo<D>(game_id: i32, db: D) -> GameInfoResp
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::games::dsl as games_dsl;
        let game: Game = games_dsl::games
            .filter(games_dsl::id.eq(game_id))
            .get_result(&db)
            .unwrap();
        let players = crate::models::Player::get_players(game_id, &db).unwrap();
        let turn = crate::models::Turn::current_turn(&[game_id], &db);
        let next_turn = match game.next_turn {
            Some(t) => match t.duration_since(std::time::SystemTime::now()) {
                Ok(t) => Some(t.as_millis() as u32),
                Err(_) => None,
            },
            None => None,
        };
        let resp = GameInfoResp {
            unk1: 0,
            game_state: if turn.len() > 0 { 2 } else { 1 },
            game_name: "aaa".to_owned(),
            era: 1,
            unk2: 0,
            disciples: false,
            unk3: 0,
            milliseconds_to_host: next_turn,
            unk4: 0,
            nation_statuses: players.iter().fold(
                std::collections::HashMap::new(),
                |mut accumulator, current| {
                    accumulator.insert(current.nationid, 1);
                    accumulator
                },
            ),
            remaining: vec![],
        };
        return resp;
    }

    fn fetch_port(&self) -> i32 {
        use crate::schema::games::dsl as games_dsl;
        let db = self.db_pool.get().unwrap();
        let game = games_dsl::games
            .filter(games_dsl::id.eq(self.game_id))
            .get_result::<Game>(&db)
            .unwrap();
        match game.port {
            Some(port) => port,
            None => {
                let port_var =
                    env::var("PORT_RANGE").expect("PORT_RANGE must be set (ie. '10000,10999')");
                let range: Vec<&str> = port_var.split(",").collect();
                let new_port: i32 = diesel::dsl::sql::<diesel::sql_types::Int4>(&format!(
                    "SELECT * FROM generate_series({}, {}) num LEFT OUTER JOIN games g ON g.port=num WHERE g.id IS NULL",
                    range[0], range[1]
                ))
                .get_result(&db)
                .unwrap();
                diesel::update(games_dsl::games.filter(games_dsl::id.eq(game.id)))
                    .set(games_dsl::port.eq(new_port))
                    .execute(&db)
                    .unwrap();
                new_port
            }
        }
    }

    fn accept_pretender<D>(game_id: i32, req: crate::packets::UploadPretenderReq, db: &D)
    where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::models::{File, NewFile, NewPlayer};
        use crate::twoh::TwoH;
        let twoh = TwoH::read_contents(&req.pretender_contents[..]).unwrap();
        let nation: Nation = Nation::get(game_id, twoh.nationid, db).unwrap();
        let file: File =
            NewFile::new(&format!("{}.2h", nation.filename), &req.pretender_contents).insert(db);

        &NewPlayer {
            file_id: file.id,
            nationid: req.nation_id as i32,
            game_id: game_id,
        }
        .insert(db)
        .unwrap();
    }

    pub fn launch(self) {
        std::thread::spawn(move || {
            let port = self.fetch_port();
            let recv_sock = std::net::TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
            loop {
                if let Ok((mut conn, client_addr)) = recv_sock.accept() {
                    let mut socket_send_clone = conn.try_clone().unwrap();
                    let tx_clone = self.bus_tx.clone();
                    let rx_clone = self.bus_rx.new_recv();
                    std::thread::spawn(move || loop {
                        let packet = Packet::from_reader(&mut conn);
                        tx_clone
                            .send(Msg::Pkt(PktMsg {
                                addr: client_addr,
                                pkt: packet.body,
                            }))
                            .unwrap();
                    });
                    let pool_clone = self.db_pool.clone();
                    let game_id = self.game_id.clone();
                    let tx_clone = self.bus_tx.clone();
                    std::thread::spawn(move || {
                        while let Ok(msg) = rx_clone.recv() {
                            match msg {
                                Msg::Pkt(PktMsg { addr, pkt }) if client_addr == addr => {
                                    log::debug!("Packet: {:x?}", pkt);
                                    match pkt {
                                        crate::packets::Body::StartGameReq(pkt) => {
                                            let db = pool_clone.get().unwrap();
                                            let game = Game::get(game_id, &db).unwrap();
                                            let schedule = std::time::SystemTime::now()
                                                .add(std::time::Duration::from_secs(5));
                                            game.schedule_turn(schedule, &db).unwrap();
                                            tx_clone
                                                .send(Msg::GameSchedule(
                                                    crate::msgbus::GameScheduleMsg {
                                                        game_id,
                                                        schedule,
                                                    },
                                                ))
                                                .unwrap();
                                        }
                                        crate::packets::Body::DisconnectReq(pkt) => {
                                            DisconnectResp {}.write_packet(&mut socket_send_clone);
                                        }
                                        crate::packets::Body::UploadPretenderReq(pkt) => {
                                            let db = pool_clone.get().unwrap();
                                            Self::accept_pretender(game_id, pkt, &db);
                                        }
                                        crate::packets::Body::HeartbeatReq(pkg) => {
                                            let db = pool_clone.get().unwrap();
                                            Self::generate_gameinfo(game_id, db)
                                                .write_packet(&mut socket_send_clone);
                                        }
                                        crate::packets::Body::AstralPacketReq(pkt) => {
                                            let resp = (AstralPacketResp {})
                                                .write_packet(&mut socket_send_clone);
                                        }
                                        crate::packets::Body::GameInfoReq(pkt) => {
                                            let db = pool_clone.get().unwrap();
                                            Self::generate_gameinfo(game_id, db)
                                                .write_packet(&mut socket_send_clone);
                                        }
                                        crate::packets::Body::PAReq(pkt) => {
                                            PAResp {}.write_packet(&mut socket_send_clone);
                                        }
                                        crate::packets::Body::PasswordsReq(pkt) => {
                                            PasswordsResp::new(&[])
                                                .write_packet(&mut socket_send_clone);
                                        }
                                        crate::packets::Body::TwoHCrcReq(pkt) => {
                                            TwoHCrcResp {}.write_packet(&mut socket_send_clone);
                                        }
                                        crate::packets::Body::TrnReq(pkt) => {
                                            log::debug!(
                                                "Client requested trn file for nation {}",
                                                pkt.nation_desired
                                            );
                                            let db = pool_clone.get().unwrap();
                                            let (_turn, file) = crate::models::PlayerTurn::get(
                                                game_id,
                                                pkt.nation_desired as i32,
                                                &db,
                                            )
                                            .unwrap();
                                            TrnResp {
                                                trn_contents: file.filebinary,
                                            }
                                            .write_packet(&mut socket_send_clone);
                                        }
                                        crate::packets::Body::MapReq(pkt) => {
                                            let db = pool_clone.get().unwrap();
                                            let game = Game::get(game_id, &db).unwrap();
                                            let map = Map::get(game.map_id, &db).unwrap();
                                            if let Ok((
                                                Some(mapfile),
                                                Some(imagefile),
                                                Some(winterfile),
                                            )) = map.get_files(&db)
                                            {
                                                MapResp {
                                                    map: Some((
                                                        mapfile.filename,
                                                        mapfile.filebinary,
                                                    )),
                                                    image: Some((
                                                        imagefile.filename,
                                                        imagefile.filebinary,
                                                    )),
                                                    winter_image: Some((
                                                        winterfile.filename,
                                                        winterfile.filebinary,
                                                    )),
                                                }
                                                .write_packet(&mut socket_send_clone);
                                            }
                                        }
                                        crate::packets::Body::MapFileReq(pkt) => {
                                            let db = pool_clone.get().unwrap();
                                            let game = Game::get(game_id, &db).unwrap();
                                            let map = Map::get(game.map_id, &db).unwrap();
                                            if let Ok((Some(mapfile), _, _)) = map.get_files(&db) {
                                                MapFileResp {
                                                    // map_contents: mapfile.filebinary,
                                                    map_contents: mapfile.filebinary
                                                }
                                                .write_packet(&mut socket_send_clone);
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                    });
                }
            }
        });
    }
}

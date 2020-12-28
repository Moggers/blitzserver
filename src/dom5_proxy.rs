use super::models::{File, Game, Map};
use crossbeam_channel::{Receiver, Sender};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

use crate::diesel::prelude::*;
use crate::dom5_proc::{Dom5Proc, Dom5ProcHandle, GameCmd};

pub struct GameMsg {
    pub id: i32,
    pub cmd: GameMsgType,
}

pub enum GameMsgType {
    GameCmd(GameCmd),
    RebootCmd,
    HostTurn,
}

pub struct Dom5Proxy {
    pub db_pool: r2d2::Pool<ConnectionManager<PgConnection>>,
    pub game_dir: std::path::PathBuf,
    pub dom5_bin: std::path::PathBuf,
    pub game_id: i32,
    pub internal_port_range: [i32; 2],
    pub manager_sender: Sender<GameMsg>,
    pub manager_receiver: Receiver<GameMsg>,
    pub port_range: [i32; 2],
    pub nextturn_interupt: Sender<()>,
    pub nextturn_interupt_recv: Receiver<()>,
}

pub struct Dom5ProxyHandle {
    pub manager_sender: Sender<GameMsg>,
}

impl Dom5Proxy {
    fn schedule_turn(&self) {
        println!("Scheduling turn");
        let recv = self.nextturn_interupt_recv.clone();
        let notifier = self.manager_sender.clone();
        let id = self.game_id;
        let db = self.db_pool.get().unwrap();
        let game: Game = {
            use crate::schema::games::dsl::*;
            games.filter(id.eq(self.game_id)).get_result(&db).unwrap()
        };
        println!("Found next turn when");
        if let Some(next_turn) = game.next_turn {
            match self
                .nextturn_interupt
                .send_timeout((), std::time::Duration::from_secs(5))
            {
                Ok(_) => {
                    println!("Sent interrupt");
                }
                Err(_) => {
                    println!("Interrupt timeout- no other schedule");
                }
            }

            println!("Begin test");
            match next_turn.duration_since(std::time::SystemTime::now()) {
                Ok(duration) => {
                    println!("Scheduling next turn in {:?}", duration);
                    std::thread::spawn(move || {
                        match recv.recv_timeout(duration) {
                            Ok(_) => {
                                println!("Next turn interrupted");
                            }
                            Err(_) => {
                                println!("Time for a new turn");
                                // Small debounce to prevent thrashing while waiting for server
                                std::thread::sleep(std::time::Duration::from_secs(5));
                                println!("Notifying");
                                notifier
                                    .send(GameMsg {
                                        id,
                                        cmd: GameMsgType::HostTurn,
                                    })
                                    .unwrap();
                            }
                        }
                    });
                }
                Err(_) => {
                    println!("Time for a new turn");
                    notifier
                        .send(GameMsg {
                            id,
                            cmd: GameMsgType::HostTurn,
                        })
                        .unwrap();
                }
            }
        }
    }

    pub fn launch(self) -> Dom5ProxyHandle {
        let db = self.db_pool.get().expect("Unable to connect to database");
        let mut game: Game = {
            use crate::schema::games::dsl::*;
            games
                .filter(id.eq(self.game_id))
                .get_result(&db)
                .expect(&format!("Faled to find game {} to launch", self.game_id))
        };
        match std::fs::remove_dir_all(&self.game_dir) {
            Ok(_) => (),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                () // File might not exist..
            }
            Err(e) => println!("Found something odd {:?}", e),
        }
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

        // Proxy listener
        let db_pool = self.db_pool.clone();
        let binpath = self.dom5_bin.clone();
        let ipr = self.internal_port_range.clone();
        let mapname = mapfile.filename.clone();
        let game_dir = self.game_dir.clone();
        let (proc_sender, proc_receiver) =
            crossbeam_channel::unbounded::<crate::dom5_proc::ProcEvent>();
        let proc_sender_clone = proc_sender.clone();

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
                                    send_upstream: proc_sender_clone.clone(),
                                    savedir: std::path::PathBuf::from(&game_dir)
                                        .join("savedgames")
                                        .join(&game.name),
                                    name: game.name.clone(),
                                    era: game.era,
                                    mapname: mapname.clone(),
                                    datadir: game_dir
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

        let receiver = self.manager_receiver.clone();
        let handle_sender = self.manager_sender.clone();
        self.schedule_turn();
        let db_pool = self.db_pool.clone();
        let game_dir = self.game_dir.clone();
        let binpath = self.dom5_bin.clone();
        let mapname = mapfile.filename.clone();
        let proc_sender_clone = proc_sender.clone();
        std::thread::spawn(move || loop {
            crossbeam_channel::select! {
                recv(proc_receiver) -> msg => {
                    match msg.unwrap() {
                        crate::dom5_proc::ProcEvent::NewTurn => {
                            println!("New turn detected in dom5proxy");
                            self.schedule_turn();
                            if let Some(d5ph) = base_proc_handle.read().unwrap().upgrade() {
                                d5ph.sender.send(crate::dom5_proc::GameCmd::SetTimerCmd).unwrap();
                            }
                        }
                    }
                }
                recv(receiver) -> msg => match msg.unwrap().cmd {
                    GameMsgType::HostTurn => {
                        println!("Here we check if the game is already running and hosting its own turns. If it is not, we fire one off");
                        if let None = base_proc_handle.read().unwrap().upgrade() {
                                let db = db_pool.get().unwrap();
                                let game: Game = {
                                    use crate::schema::games::dsl::*;
                                    games.filter(id.eq(self.game_id)).get_result(&db).unwrap()
                                };
                                Dom5Proc {
                                    game_id: game.id,
                                    send_upstream:proc_sender_clone.clone(), 
                                    savedir: std::path::PathBuf::from(&game_dir)
                                        .join("savedgames")
                                        .join(&game.name),
                                    name: game.name.clone(),
                                    era: game.era,
                                    mapname: mapname.clone(),
                                    datadir: game_dir
                                        .clone()
                                        .into_os_string()
                                        .into_string()
                                        .unwrap(),
                                    db_pool: db_pool.clone(),
                                    internal_port_range: ipr,
                                }
                                .host_turn(&binpath);

                        } else {
                            self.schedule_turn();
                        }
                    }
                    GameMsgType::RebootCmd => match base_proc_handle.read().unwrap().upgrade() {
                        Some(d5ph) => {
                            d5ph.sender.send(GameCmd::Shutdown).unwrap();
                        }
                        None => {
                            println!("Attempted to destroy, but already dead");
                        }
                    },
                    GameMsgType::GameCmd(GameCmd::SetTimerCmd) => {
                        println!("Received set timer cmd, scheduling turn");
                        self.schedule_turn();
                    },
                    GameMsgType::GameCmd(game_cmd) => {
                        if let Some(d5ph) = base_proc_handle.read().unwrap().upgrade() {
                            d5ph.sender.send(game_cmd).unwrap();
                        }
                    }
                },
            }
        });
        Dom5ProxyHandle {
            manager_sender: handle_sender,
        }
    }
}

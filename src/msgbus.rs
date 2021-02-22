use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub enum Msg {
    Pkt(PktMsg),
    GameSchedule(GameScheduleMsg),
}

#[derive(Clone)]
pub struct PktMsg {
    pub addr: std::net::SocketAddr,
    pub pkt: crate::packets::Body,
}
#[derive(Clone)]
pub struct GameScheduleMsg {
    pub game_id: i32,
    pub schedule: std::time::SystemTime,
}

pub struct MsgBus {
    broadcasts: Arc<Mutex<Vec<crossbeam_channel::Sender<Msg>>>>,
    pub sender: crossbeam_channel::Sender<Msg>,
}

impl MsgBus {
    pub fn new() -> MsgBus {
        let (tx, rx) = crossbeam_channel::unbounded::<Msg>();
        let broadcasts: Arc<Mutex<Vec<crossbeam_channel::Sender<Msg>>>> =
            Arc::new(Mutex::new(vec![]));
        let cloned_broadcast = broadcasts.clone();
        std::thread::spawn(move || loop {
            if let Ok(m) = rx.recv() {
                if let Ok(mut broadcasters) = cloned_broadcast.lock() {
                    broadcasters.drain_filter(|broadcaster| !broadcaster.send(m.clone()).is_ok());
                }
            }
        });
        MsgBus {
            broadcasts,
            sender: tx,
        }
    }
    pub fn new_recv(&self) -> MsgBusRx {
        let (tx, rx) = crossbeam_channel::unbounded::<Msg>();
        if let Ok(mut tx_vec) = (*self.broadcasts).lock() {
            tx_vec.push(tx);
            drop(tx_vec);
            MsgBusRx {
                rx,
                broadcasts: self.broadcasts.clone(),
            }
        } else {
            panic!("Fail case not implemented");
        }
    }

    pub fn send(&self, msg: Msg) {
        self.sender.send(msg).unwrap();
    }
}

pub struct MsgBusRx {
    rx: crossbeam_channel::Receiver<Msg>,
    broadcasts: Arc<Mutex<Vec<crossbeam_channel::Sender<Msg>>>>,
}

impl MsgBusRx {
    pub fn new_recv(&self) -> MsgBusRx {
        let (tx, rx) = crossbeam_channel::unbounded::<Msg>();
        if let Ok(mut tx_vec) = (*self.broadcasts).lock() {
            tx_vec.push(tx);
            drop(tx_vec);
            MsgBusRx {
                rx,
                broadcasts: self.broadcasts.clone(),
            }
        } else {
            panic!("Fail case not implemented");
        }
    }
    pub fn recv(&self) -> Result<Msg, crossbeam_channel::RecvError> {
        self.rx.recv()
    }
    pub fn recv_timeout(&self, timeout: std::time::Duration) -> Result<Msg, crossbeam_channel::RecvTimeoutError> {
        self.rx.recv_timeout(timeout)
    }
}

pub type MsgBusTx = crossbeam_channel::Sender<Msg>;
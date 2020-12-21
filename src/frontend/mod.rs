use diesel::r2d2::ConnectionManager;
use super::game_manager::ManagerMsg;
use diesel::PgConnection;

pub mod mods;
pub mod maps;
pub mod games;

#[derive(Clone)]
pub struct AppData {
    pub pool: r2d2::Pool<ConnectionManager<PgConnection>>,
    pub manager_notifier: crossbeam_channel::Sender<ManagerMsg>,
}


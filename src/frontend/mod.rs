use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use crate::msgbus::MsgBusTx;

pub mod mods;
pub mod maps;
pub mod games;
pub mod utils;

#[derive(Clone)]
pub struct AppData {
    pub pool: r2d2::Pool<ConnectionManager<PgConnection>>,
    pub msgbus_sender: MsgBusTx,
    pub email_manager: crate::email_manager::EmailManager
}


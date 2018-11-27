use actix_web::actix::Addr;
use crate::db::DbExecutor;

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

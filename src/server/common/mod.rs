use crate::db::DbExecutor;
use actix_web::actix::Addr;

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

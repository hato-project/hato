use actix_web::actix::Addr;
use db::DbExecutor;

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

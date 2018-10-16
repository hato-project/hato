use actix_web::actix::Addr;
use db::ConnDsl;

pub struct AppState {
    pub db: Addr<ConnDsl>,
}

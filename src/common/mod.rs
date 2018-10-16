use actix_web::actix::Addr;
use db::ConnDsl;

pub struct AppContext {
    pub db: Addr<ConnDsl>,
}

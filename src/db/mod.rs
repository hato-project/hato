#![allow(warnings)]
pub mod schema;

use actix_web::actix::{Actor, Addr, SyncArbiter, SyncContext};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv;
use num_cpus;

pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub fn init() -> Addr<DbExecutor> {
    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let conn = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    SyncArbiter::start(num_cpus::get() * 4, move || DbExecutor(conn.clone()))
}

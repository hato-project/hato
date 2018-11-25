#![allow(warnings)]

use actix_web::actix::{Actor, Addr, SyncArbiter, SyncContext};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv;
use num_cpus;

pub mod schema;

pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub fn init(db_url: &str) -> Addr<DbExecutor> {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let conn = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    SyncArbiter::start(num_cpus::get() * 4, move || DbExecutor(conn.clone()))
}

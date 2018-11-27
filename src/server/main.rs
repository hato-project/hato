#![cfg_attr(feature = "cargo-clippy", allow(clippy::needless_pass_by_value))]

extern crate actix_web;
extern crate chrono;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
#[macro_use]
extern crate failure;
extern crate futures;
extern crate listenfd;
#[macro_use]
extern crate log;
extern crate num_cpus;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use actix_web::{actix::System, server};
use clap::App;
use crate::common::AppState;
use listenfd::ListenFd;

mod api;
mod builder;
mod common;
mod config;
mod db;
mod errors;
mod handler;
mod model;
mod router;

fn main() {
    dotenv::dotenv().ok();

    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).version(crate_version!()).get_matches();
    let cfg = config::init_server_config(&matches);
    run_server(cfg);
}

fn run_server(cfg: config::ServerConfig) {
    info!("Starting hato...");

    let sys = System::new("hato");
    let mut listenfd = ListenFd::from_env();

    let addr = db::init(cfg.db_url);

    let mut server = server::new(move || {
        vec![
            router::app_hato(AppState { db: addr.clone() }).boxed(),
            router::app_common().boxed(),
        ]
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind(cfg.listen_at).unwrap()
    };

    server.shutdown_timeout(2).start();

    sys.run();
}

#![cfg_attr(feature = "cargo-clippy", allow(clippy::needless_pass_by_value))]

#[macro_use]
extern crate diesel;
extern crate actix_web;
extern crate dotenv;
extern crate env_logger;
extern crate num_cpus;
#[macro_use]
extern crate log;
extern crate futures;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate failure;

mod api;
mod builder;
mod common;
mod db;
mod errors;
mod handler;
mod model;
mod router;

use actix_web::{actix::System, server};

#[macro_use]
extern crate clap;
use clap::{App, AppSettings, SubCommand};

extern crate listenfd;

use common::AppState;
use listenfd::ListenFd;

fn main() {
    dotenv::dotenv().ok();

    let app = App::new("hato")
        .version(crate_version!())
        .about("Let the Hato Fly.")
        .author("Hato Project")
        .subcommand(SubCommand::with_name("server").about("Main server of hato"))
        .subcommand(SubCommand::with_name("builder").about("Builder of hato"))
        .setting(AppSettings::ArgRequiredElseHelp);

    let matches = app.get_matches();
    match matches.subcommand() {
        ("server", Some(_server_matches)) => {
            run_server();
        }
        ("builder", Some(_builder_matches)) => {
            run_builder();
        }
        (_, _) => {}
    }
}

fn run_builder() {
    builder::docker::run_command();
    println!("hey i'm builder!");
}

fn run_server() {
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");

    env_logger::init_from_env(env);

    info!("Starting hato...");

    let sys = System::new("hato");

    let mut listenfd = ListenFd::from_env();

    let addr = db::init();

    let mut server = server::new(move || {
        vec![
            router::app_hato(AppState { db: addr.clone() }).boxed(),
            router::app_common().boxed(),
        ]
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind("0.0.0.0:8000").unwrap()
    };

    server.shutdown_timeout(2).start();

    sys.run();
}

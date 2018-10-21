#![cfg_attr(
    feature = "cargo-clippy",
    allow(clippy::needless_pass_by_value)
)]

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

mod api;
mod common;
mod db;
mod handler;
mod model;
mod router;

use actix_web::{actix::System, server};

#[macro_use]
extern crate clap;
use clap::{Arg, App, SubCommand, AppSettings}; 

fn main() {
    let app = App::new("hato")
    .version(crate_version!())
    .about("Let the Hato Fly.")
    .author("Hato Project")
    .subcommand(SubCommand::with_name("server")
                .about("Main server of hato"))
    .subcommand(SubCommand::with_name("builder")
                .about("Builder of hato"))
    .setting(AppSettings::ArgRequiredElseHelp);
    
    let matches = app.get_matches();
    match matches.subcommand() {
        ("server", Some(_server_matches)) => {run_server();}
        ("builder", Some(_builder_matches)) => {run_builder();}
        (_,_) => {}
    }

}

fn run_builder() {
    println!("hey i'm builder!");
}

fn run_server() {
    env_logger::init();

    info!("Starting hato...");

    let sys = System::new("hato");

    server::new(move || vec![router::app_hato().boxed(), router::app_common().boxed()])
        .bind("0.0.0.0:8000")
        .unwrap()
        .shutdown_timeout(2)
        .start();

    sys.run();
}

extern crate simplelog;

use clap::ArgMatches;

#[derive(Debug, Copy, Clone)]
pub struct ServerConfig<'a> {
    pub common_config: CommonConfig,
    pub listen_at:     &'a str,
    pub db_url:        &'a str,
}

#[derive(Debug, Copy, Clone)]
pub struct BuilderConfig {
    pub common_config: CommonConfig,
}

#[derive(Debug, Copy, Clone)]
pub struct CommonConfig {
    pub verbose: simplelog::LevelFilter,
}

fn init_common_config(matches: &ArgMatches) -> CommonConfig {
    let level = match matches.value_of("log_level") {
        None | Some("error") => simplelog::LevelFilter::Error,
        Some("warn") => simplelog::LevelFilter::Warn,
        Some("info") => simplelog::LevelFilter::Info,
        Some("debug") => simplelog::LevelFilter::Debug,
        Some("trace") => simplelog::LevelFilter::Trace,
        unknown => panic!("{:?} log level", unknown),
    };

    println!("current log level: {:?}", level);
    simplelog::TermLogger::init(level, simplelog::Config::default())
        .expect("failed to init logger");

    CommonConfig { verbose: level }
}

pub fn init_server_config<'a>(matches: &'a ArgMatches<'a>) -> ServerConfig<'a> {
    let listen_at = matches.value_of("listen_at").unwrap();
    let db_url = matches.value_of("db_url").unwrap();

    ServerConfig {
        common_config: init_common_config(matches),
        listen_at,
        db_url,
    }
}

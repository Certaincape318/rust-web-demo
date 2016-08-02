extern crate fern;
extern crate time;
extern crate postgres;
extern crate rustc_serialize;
extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate crypto;
extern crate session;
extern crate iron_params as params;
extern crate config;
extern crate redis;
extern crate bincode;
extern crate r2d2_redis;
extern crate persistent_time;
extern crate template;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;


pub mod controllers;
pub mod repository;
pub mod models;
pub mod utils;
pub mod services;
pub mod schedule;
pub mod cache;

use iron::prelude::*;
use std::net::*;

fn main() {
    let logger_config = fern::DispatchConfig {
        format: Box::new(|msg: &str, level: &log::LogLevel, location: &log::LogLocation| {
            format!("{} {} {} {}",
                    time::now().strftime("%Y-%m-%d %H:%M:%S").unwrap(),
                    location.module_path(),
                    level,
                    msg)

        }),
        output: vec![fern::OutputConfig::stdout()],
        level: log::LogLevelFilter::Debug,
    };
    if let Err(e) = fern::init_global_logger(logger_config, log::LogLevelFilter::Debug) {
        panic!("Failed to initialize global logger: {}", e);

    }

    schedule::init();
    let port = utils::config::Config::default().get_i32("web.listen.port");
    let host = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port as u16);
    match Iron::new(controllers::get_chain()).http(host) {
        Ok(http) => {
            info!("Success starting iron http server:{:?}", http);
        }
        Err(err) => {
            panic!("Error starting iron. The error is:{}", err);
        }
    }
}

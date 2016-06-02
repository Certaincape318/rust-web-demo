extern crate postgres;
extern crate rustc_serialize;
extern crate iron;
extern crate persistent;
extern crate router;
extern crate mount;
extern crate urlencoded;
extern crate staticfile;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate time;
extern crate handlebars_iron as hbs;
extern crate term;
extern crate logger;
extern crate crypto;
extern crate hyper;
extern crate chrono;
extern crate iron_login;
extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate config;
extern crate redis;
extern crate bincode;
extern crate r2d2_redis;
#[macro_use]
extern crate log;

pub mod controllers;
pub mod repository;
pub mod models;
pub mod utils;
pub mod services;
pub mod schedule;
pub mod cache;


pub fn run() {
    use iron::prelude::*;
    use std::net::*;
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

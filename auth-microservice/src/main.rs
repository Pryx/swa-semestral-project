#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate juniper;
extern crate config;

use std::io;

use actix_web::{middleware, App,HttpServer};
use actix_web_prom::PrometheusMetrics;
use std::collections::HashMap;
use crate::handlers::*;

pub mod schema;
pub mod actions;
pub mod model;
pub mod db;
pub mod handlers;
pub mod users;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,diesel=debug");
    env_logger::init();

    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("config")).unwrap()
        .merge(config::Environment::with_prefix("AUTH_MICRO")).unwrap();

    let config = settings.try_into::<HashMap<String, String>>().unwrap();

    let port; 
    if config.contains_key("port") {
        port = config["port"].clone();
    } else {
        port = String::from("12345");
    }

    let prometheus = PrometheusMetrics::new("api", Some("/metrics"), None);

    let link = format!("0.0.0.0:{}", port);

    let pool = db::establish_connection();
    println!("AUTH microservice running at http://{}", link);
    HttpServer::new(move || {
        App::new()     
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .wrap(prometheus.clone())
            .service(get_user)
            .service(login_user)
            .service(logout_user)
            .service(logged_in)
            .service(register_user)
            .service(update_user)
    })
    .bind(&link)?
    .run().await
}

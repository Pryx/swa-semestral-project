#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate juniper;
extern crate config;

use std::io;

use actix_web::{get, post, web, middleware, App, Error, HttpResponse, HttpServer};
use actix_web::dev::HttpResponseBuilder;
use actix_web_prom::PrometheusMetrics;
use std::collections::HashMap;
use http::StatusCode;

pub mod schema;
pub mod actions;
pub mod model;
pub mod db;

/// Finds user by UID.
#[get("/user/{user_id}")]
async fn get_user(
    pool: web::Data<db::PgPool>,
    user_uid: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let user_uid = user_uid.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let res = web::block(move || actions::find_user_by_uid(user_uid, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    return Ok(HttpResponseBuilder::new(StatusCode::from_u16(res.code).unwrap()).json(res));
}

#[post("/login/")]
async fn login_user(
    pool: web::Data<db::PgPool>,
    data: web::Json<model::Login>
) -> Result<HttpResponse, Error> {

    let conn = pool.get().expect("couldn't get db connection from pool");
    // use web::block to offload blocking Diesel code without blocking server thread
    let token = web::block(move || actions::login(data.into_inner(), &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    return Ok(HttpResponseBuilder::new(StatusCode::from_u16(token.code).unwrap()).json(token));
}

#[post("/logout/")]
async fn logout_user(
    pool: web::Data<db::PgPool>,
    data: web::Json<model::TokenInfo>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let res = web::block(move || actions::logout(data.into_inner(), &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    return Ok(HttpResponseBuilder::new(StatusCode::from_u16(res.code).unwrap()).json(res));
}

#[post("/register/")]
async fn register_user(
    pool: web::Data<db::PgPool>,
    data: web::Json<model::RegisterUser>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || actions::register(data.into_inner(), &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

        
    return Ok(HttpResponseBuilder::new(StatusCode::from_u16(user.code).unwrap()).json(user));
}

#[post("/logged_in/")]
async fn logged_in(
    pool: web::Data<db::PgPool>,
    data: web::Json<model::TokenInfo>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let res = web::block(move || actions::verify_token(data.into_inner(), &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    
    return Ok(HttpResponseBuilder::new(StatusCode::from_u16(res.code).unwrap()).json(res));
}

#[post("/update/")]
async fn update_user(
    pool: web::Data<db::PgPool>,
    data: web::Json<model::UpdateRequest>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let res = web::block(move || actions::update(data.into_inner(), &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    
    return Ok(HttpResponseBuilder::new(StatusCode::from_u16(res.code).unwrap()).json(res));
}

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

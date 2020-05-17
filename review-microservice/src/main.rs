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

/// Finds review by UID.
#[get("/review/{review_id}")]
async fn get_review(
    pool: web::Data<db::PgPool>,
    review_uid: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let review_uid = review_uid.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let res = web::block(move || actions::find_review_by_uid(review_uid, &conn))
        .await;

    match res {
        Ok(r) => Ok(HttpResponseBuilder::new(StatusCode::from_u16(r.code).unwrap()).json(r)),
        Err(_) =>{
            let msg: model::Response<String> = model::Response{
                success: false,
                data: None,
                message: format!("Internal server error!"),
                code: 500
            };

            return Ok(HttpResponse::InternalServerError().json(msg));
        }
    }
}


#[get("/reviews/product/{product_id}")]
async fn get_reviews_for_product(
    pool: web::Data<db::PgPool>,
    product_uid: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let product_uid = product_uid.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let res = web::block(move || actions::find_reviews_for_product(product_uid, &conn))
        .await;

    match res {
        Ok(r) => Ok(HttpResponseBuilder::new(StatusCode::from_u16(r.code).unwrap()).json(r)),
        Err(_) =>{
            let msg: model::Response<String> = model::Response{
                success: false,
                data: None,
                message: format!("Internal server error!"),
                code: 500
            };

            return Ok(HttpResponse::InternalServerError().json(msg));
        }
    }
}

#[get("/reviews/user/{user_id}")]
async fn get_reviews_for_user(
    pool: web::Data<db::PgPool>,
    user_uid: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let user_uid = user_uid.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let res = web::block(move || actions::find_reviews_for_user(user_uid, &conn))
        .await;

    match res {
        Ok(r) => Ok(HttpResponseBuilder::new(StatusCode::from_u16(r.code).unwrap()).json(r)),
        Err(_) =>{
            let msg: model::Response<String> = model::Response{
                success: false,
                data: None,
                message: format!("Internal server error!"),
                code: 500
            };

            return Ok(HttpResponse::InternalServerError().json(msg));
        }
    }
}

#[post("/add/")]
async fn add_review(
    pool: web::Data<db::PgPool>,
    data: web::Json<model::AddReview>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let review = web::block(move || actions::add(data.into_inner(), &conn))
        .await;

    match review {
        Ok(r) => Ok(HttpResponseBuilder::new(StatusCode::from_u16(r.code).unwrap()).json(r)),
        Err(_) =>{
            let msg: model::Response<String> = model::Response{
                success: false,
                data: None,
                message: format!("Internal server error!"),
                code: 500
            };

            return Ok(HttpResponse::InternalServerError().json(msg));
        }
    }
}

#[post("/update/{reviewId}")]
async fn update_review(
    pool: web::Data<db::PgPool>,
    review_uid: web::Path<i32>,
    data: web::Json<model::UpdateRequest>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let review_uid = review_uid.into_inner();

    // use web::block to offload blocking Diesel code without blocking server thread
    let res = web::block(move || actions::update(review_uid, data.into_inner(), &conn))
        .await;

    
    match res {
        Ok(r) => Ok(HttpResponseBuilder::new(StatusCode::from_u16(r.code).unwrap()).json(r)),
        Err(_) =>{
            let msg: model::Response<String> = model::Response{
                success: false,
                data: None,
                message: format!("Internal server error!"),
                code: 500
            };

            return Ok(HttpResponse::InternalServerError().json(msg));
        }
    }
}





#[post("/delete/{reviewId}")]
async fn delete_review(
    pool: web::Data<db::PgPool>,
    review_uid: web::Path<i32>,
    data: web::Json<model::DeleteRequest>
) -> Result<HttpResponse, Error> {

    let review_uid = review_uid.into_inner();

    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let res = web::block(move || actions::delete(review_uid, data.into_inner(), &conn))
        .await;

    
    match res {
        Ok(r) => Ok(HttpResponseBuilder::new(StatusCode::from_u16(r.code).unwrap()).json(r)),
        Err(_) =>{
            let msg: model::Response<String> = model::Response{
                success: false,
                data: None,
                message: format!("Internal server error!"),
                code: 500
            };

            return Ok(HttpResponse::InternalServerError().json(msg));
        }
    }
}









#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,diesel=debug");
    env_logger::init();

    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("config")).unwrap()
        .merge(config::Environment::with_prefix("REVIEW_MICRO")).unwrap();

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

    println!("REVIEW microservice running at http://{}", link);
    HttpServer::new(move || {
        App::new()     
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .wrap(prometheus.clone())
            .service(get_review)
            .service(add_review)
            .service(delete_review)
            .service(update_review)
            .service(get_reviews_for_product)
            .service(get_reviews_for_user)

    })
    .bind(&link)?
    .run().await
}
 
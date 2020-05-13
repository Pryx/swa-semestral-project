use actix_web::{get, post, web, Error, HttpResponse};
use actix_web::dev::HttpResponseBuilder;
use http::StatusCode;

use crate::model;
use crate::actions;
use crate::db;
use crate::users;

/// Finds user by UID.
#[get("/user/{user_id}")]
async fn get_user(
    pool: web::Data<db::PgPool>,
    user_uid: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let user_uid = user_uid.into_inner();
    let service = users::UserService{pool: pool.into_inner().clone()};
    // use web::block to offload blocking Diesel code without blocking server thread
    let res = web::block(move || actions::find_user_by_uid(user_uid, &service))
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

#[post("/login/")]
async fn login_user(
    pool: web::Data<db::PgPool>,
    data: web::Json<model::Login>
) -> Result<HttpResponse, Error> {

    let service = users::UserService{pool: pool.into_inner().clone()};
    // use web::block to offload blocking Diesel code without blocking server thread
    let token = web::block(move || actions::login(data.into_inner(), &service))
        .await;

    match token {
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

#[post("/logout/")]
async fn logout_user(
    pool: web::Data<db::PgPool>,
    data: web::Json<model::TokenInfo>
) -> Result<HttpResponse, Error> {
    let service = users::UserService{pool: pool.into_inner().clone()};

    // use web::block to offload blocking Diesel code without blocking server thread
    let res = web::block(move || actions::logout(data.into_inner(), &service))
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

#[post("/register/")]
async fn register_user(
    pool: web::Data<db::PgPool>,
    data: web::Json<model::RegisterUser>
) -> Result<HttpResponse, Error> {
    let service = users::UserService{pool: pool.into_inner().clone()};

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || actions::register(data.into_inner(), &service))
        .await;

    match user {
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

#[post("/logged_in/")]
async fn logged_in(
    pool: web::Data<db::PgPool>,
    data: web::Json<model::TokenInfo>
) -> Result<HttpResponse, Error> {
    let service = users::UserService{pool: pool.into_inner().clone()};

    // use web::block to offload blocking Diesel code without blocking server thread
    let res = web::block(move || actions::verify_token(data.into_inner(), &service))
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

#[post("/update/")]
async fn update_user(
    pool: web::Data<db::PgPool>,
    data: web::Json<model::UpdateRequest>
) -> Result<HttpResponse, Error> {
    let service = users::UserService{pool: pool.into_inner().clone()};

    // use web::block to offload blocking Diesel code without blocking server thread
    let res = web::block(move || actions::update(data.into_inner(), &service))
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
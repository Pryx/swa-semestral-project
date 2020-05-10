use serde::{Deserialize, Serialize};

use crate::schema::*;


#[derive(Debug, Clone, Serialize, Insertable, Queryable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub pass: String,
    #[serde(skip_serializing)]
    pub tokens: Option<Vec<String>>
}


#[derive(Serialize, Deserialize)]
pub struct Login {
    pub email: String,
    pub pass: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenInfo {
    pub email: String,
    pub token: String,
}


#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    pub data: Option<T>,
    pub message: String,
    pub success: bool,
    #[serde(skip_serializing)]
    pub code: u16
}


#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct RegisterUser {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub pass: String,
}

#[derive(Serialize, Deserialize,)]
pub struct UpdateRequest {
    pub email: String,
    pub token: String,
    pub user_data: RegisterUser,
}
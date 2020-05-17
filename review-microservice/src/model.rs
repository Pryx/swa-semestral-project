use serde::{Deserialize, Serialize};

use crate::schema::*;


#[derive(Debug, Clone, Serialize, Insertable, Queryable)]
#[table_name = "reviews"]
pub struct Review {
    pub id: i32,
    pub user_id: i32,
    pub review_text: Option<String>,
    pub product_id: String,
    pub created: i32,
    pub rating: i32
}

/*
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

*/
#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    pub data: Option<T>,
    pub message: String,
    pub success: bool,
    #[serde(skip_serializing)]
    pub code: u16
}


#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "reviews"]
pub struct AddReview {
    pub user_id: i32,
    pub review_text: Option<String>,
    pub product_id: String,
    pub created: i32,
    pub rating: i32
}

#[derive(Serialize, Deserialize,)]
pub struct UpdateRequest {
    pub email: String,
    pub token: String,
    pub user_data: AddReview,
}



#[derive(Serialize, Deserialize,)]
pub struct DeleteRequest {
    pub email: String,
    pub token: String,
}

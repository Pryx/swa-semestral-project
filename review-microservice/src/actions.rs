use crate::model;
extern crate diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

//use diesel::dsl::count_star;


/// Run query using Diesel to insert a new database row and return the result.
pub fn find_review_by_uid(
    uid: i32,
    conn: &PgConnection,
) -> Result<model::Response<model::Review>, diesel::result::Error> {
    use crate::schema::reviews::dsl::*;

    let review = reviews.
        filter(id.eq(uid))
        .first::<model::Review>(conn)
        .optional()?;

    match review {
        Some(u) => return Ok(
            model::Response{
                success: true,
                data: Some(u),
                message: format!("Review found!"),
                code: 200
            }
        ),
        None => return Ok(model::Response{
                success: false,
                data: None,
                message: format!("Review not found!"),
                code: 404
            }
        )
    }
}


pub fn find_by_id(
    r_id: i32,
    conn: &PgConnection,
) -> Result<Option<model::Review>, diesel::result::Error> {
    use crate::schema::reviews::dsl::*;

    let review = reviews.
        filter(id.eq(r_id))
        .first::<model::Review>(conn)
        .optional()?;

    Ok(review)
}


pub fn find_reviews_for_product(
    uid: String,
    conn: &PgConnection,
) -> Result<model::Response<Vec<model::Review>>, diesel::result::Error> {
    use crate::schema::reviews::dsl::*;

    let review = reviews.
        filter(product_id.eq(uid))
        .load(conn)
        .optional()?;

    match review {
        Some(u) => return Ok(
            model::Response{
                success: true,
                data: Some(u),
                message: format!("Reviews found!"),
                code: 200
            }
        ),
        None => return Ok(model::Response{
                success: false,
                data: None,
                message: format!("Reviews not found!"),
                code: 404
            }
        )
    }
}


pub fn find_reviews_for_user(
    uid: i32,
    conn: &PgConnection,
) -> Result<model::Response<Vec<model::Review>>, diesel::result::Error> {
    use crate::schema::reviews::dsl::*;

    let review = reviews.
        filter(user_id.eq(uid))
        .load(conn)
        .optional()?;

    match review {
        Some(u) => return Ok(
            model::Response{
                success: true,
                data: Some(u),
                message: format!("Reviews found!"),
                code: 200
            }
        ),
        None => return Ok(model::Response{
                success: false,
                data: None,
                message: format!("Reviews not found!"),
                code: 404
            }
        )
    }
}



pub fn add(
    mut data: model::AddReview,
    conn: &PgConnection,
) -> Result<model::Response<model::Review>, diesel::result::Error> {
    use crate::schema::reviews::dsl::reviews;
    extern crate diesel;
    //let email = data.email.clone();



/*
    let found = find_user_by_email(email.clone(), conn);

    if let Ok(Some(_)) = found{
        return Ok(
            model::Response{
                success: false,
                message: format!("User already exists!"),
                data: None,
                code: 400
            }
        ); 
    }
*/

    let review_added: Result<model::Review, diesel::result::Error> = diesel::insert_into(reviews)
        .values(data)
        .get_result(conn);

    match review_added {
        Ok(row) => {
            if row.id >= 1 {
                let review = find_by_id(row.id, conn);
                match review {
                    Ok(usr) => return Ok(
                        model::Response{
                            success: true,
                            message: format!("Successfully added review"),
                            data: usr,
                            code: 200
                        }
                    ),
                    Err(_) => return Ok(
                        model::Response{
                            success: false,
                            message: format!("Error adding review!"),
                            data: None,
                            code: 500
                        }
                    )
                }
            }else{
                return Ok(
                        model::Response{
                            success: false,
                            message: format!("User already reviewed this product!"),
                            data: None,
                            code: 400
                        }
                    );
            }
        },
        Err(e) => return Err(e)
    }
}


pub fn delete(
    r_id: i32,
    mut data: model::DeleteRequest,
    conn: &PgConnection,
) -> Result<model::Response<model::Review>, diesel::result::Error> {
    use crate::schema::reviews::dsl::*;
    //let email = data.email.clone();



/*
    let found = find_user_by_email(email.clone(), conn);

    if let Ok(Some(_)) = found{
        return Ok(
            model::Response{
                success: false,
                message: format!("User already exists!"),
                data: None,
                code: 400
            }
        ); 
    }
*/

    let rows_affected = diesel::delete(reviews.filter(id.eq(r_id))) 
        .execute(conn);

    match rows_affected {
        Ok(rows) => {
            if rows == 1 {
                let review = find_by_id(1, conn);
                match review {
                    Ok(usr) => return Ok(
                        model::Response{
                            success: true,
                            message: format!("Successfully deleted review"),
                            data: None,
                            code: 200
                        }
                    ),
                    Err(_) => return Ok(
                        model::Response{
                            success: false,
                            message: format!("Error deleting review!"),
                            data: None,
                            code: 500
                        }
                    )
                }
            }else{
                return Ok(
                        model::Response{
                            success: false,
                            message: format!("Review not found!"),
                            data: None,
                            code: 400
                        }
                    );
            }
        },
        Err(e) => return Err(e)
    }
}


pub fn update(
    uid: i32,
    data: model::UpdateRequest,
    conn: &PgConnection,
) -> Result<model::Response<model::Review>, diesel::result::Error> {
    
    use crate::schema::reviews::dsl::*;

   /* let is_logged = verify_token(model::TokenInfo{
            email: data.email.clone(),
            token: data.token.clone()
        }, conn);

    if let Ok(is_logged) = is_logged {
        if is_logged.success && is_logged.code == 200 {

            let mut hasher = Sha256::new();
            hasher.input_str(&data.user_data.pass);
            let passwd = hasher.result_str();
*/
            let rows_affected = diesel::update(reviews).filter(id.eq(uid))
                .set(
                    (
                        user_id.eq(data.user_data.user_id),
                        review_text.eq(data.user_data.review_text),
                        product_id.eq(data.user_data.product_id),
                        created.eq(data.user_data.created),
                        rating.eq(data.user_data.rating),
                    )
                )
                .execute(conn);

            match rows_affected {
                Ok(v) => {
                    if v == 1 {
                        let updated_usr = find_by_id(uid, conn);
                        return Ok(
                            model::Response{
                                success: true,
                                message: format!("Successfully updated review!"),
                                data: updated_usr.unwrap(),
                                code: 200
                            }
                        )
                    }else{
                        return Ok(
                            model::Response{
                                success: false,
                                message: format!("Review not found."),
                                data: None,
                                code: 400
                            }
                        )
                    }
                },
                Err(e) => return Err(e)
            }
      /*  } else {
            return Ok(
                model::Response{
                    success: is_logged.success,
                    message: is_logged.message,
                    data: None,
                    code: is_logged.code
                }
            )
        }
    } else {
        return Ok(
            model::Response{
                success: false,
                message: format!("Internal error!"),
                data: None,
                code: 500
            }
        )
    }
    */

}


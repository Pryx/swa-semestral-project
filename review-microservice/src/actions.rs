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


/*
pub fn login(
    data: model::Login,
    conn: &PgConnection,
) -> Result<model::Response<String>, diesel::result::Error> {
    use crate::schema::users::dsl::users;
    use crate::schema::users::dsl::id;
    let email = data.email.clone();
    let user = find_user_by_email(email, conn);

    match user {
        Ok(usr) => {
            if let Some(usr) = usr {
                let mut hasher = Sha256::new();
                hasher.input_str(&data.pass);
                let passwd = hasher.result_str();

                if passwd == usr.pass{
                    let mut tokens_vec;
                    if let Some(tokens) = usr.tokens {
                        tokens_vec = tokens;
                    } else{
                        tokens_vec = vec!();
                    }

                    let now = SystemTime::now();
                    hasher = Sha256::new();
                    let dur = now.duration_since(UNIX_EPOCH)
                        .expect("Time went backwards");
                    hasher.input(format!("{}",dur.as_secs()).as_bytes());

                    let result = hasher.result_str();
                    tokens_vec.push(result.clone());

                    let res = 
                        diesel::update(users.filter(id.eq(usr.id)))
                        .set(crate::schema::users::tokens.eq(tokens_vec))
                        .execute(conn);

                    match res {
                        Ok(v) => {
                            if v == 1{
                                return Ok(
                                    model::Response{
                                        success: true,
                                        data: Some(format!("{}", result.clone())),
                                        message: format!("Successfully logged in!"),
                                        code: 200
                                    }
                                );
                            } else{
                                return Ok(
                                    model::Response{
                                        success: false,
                                        data: Some(format!("{}", result.clone())),
                                        message: format!("Error inserting token!"),
                                        code: 500
                                    }
                                );
                            }
                        }
                        ,
                        Err(e) => return Err(e)
                    }
                }else{
                    return Ok(model::Response{
                                success: false,
                                data: None,
                                message: format!("Wrong credentials!"),
                                code: 401
                            }
                        );
                }
            } else {
                return Ok(model::Response{
                            success: false,
                            data: None,
                            message: format!("User not found!"),
                            code: 400
                        }
                    );
            }
        },
        Err(e) => return Err(e)
    }
}

pub fn logout(
    data: model::TokenInfo,
    conn: &PgConnection,
) -> Result<model::Response<bool>, diesel::result::Error> {
    use crate::schema::users::dsl::users;
    use crate::schema::users::dsl::id;
    let email = data.email.clone();
    let token = data.token.clone();
    let user = find_user_by_email(email, conn);
    match user {
        Ok(usr) => {
            if let Some(usr) = usr {
                if let Some(mut tokens) = usr.tokens {
                    if let Some(pos) = tokens.iter().position(|x| *x == token) {
                        tokens.remove(pos);
                    }else{
                        return Ok(model::Response{
                                    success: true,
                                    data: Some(false),
                                    message: format!("Token not found!"),
                                    code: 400
                                }
                            );
                    }

                    let res: Result<model::User, diesel::result::Error> = 
                        diesel::update(users.filter(id.eq(usr.id)))
                        .set(crate::schema::users::tokens.eq(tokens))
                        .get_result(conn);

                    match res {
                        Ok(_) => return Ok(
                                model::Response{
                                    success: true,
                                    data: Some(true),
                                    message: format!("OK"),
                                    code: 200
                                }
                            ),
                        Err(e) => return Err(e)
                    }
                } else{
                    return Ok(model::Response{
                                success: true,
                                data: Some(false),
                                message: format!("User has no tokens!"),
                                code: 400
                            }
                        );
                }
            } else {
                return Ok(model::Response{
                            success: false,
                            data: None,
                            message: format!("User not found!"),
                            code: 400
                        }
                    );
            }
        },
        Err(e) => return Err(e)
    }
}

pub fn verify_token(
    data: model::TokenInfo,
    conn: &PgConnection,
) -> Result<model::Response<bool>, diesel::result::Error> {
    let email = data.email.clone();
    let token = data.token.clone();
    let user = find_user_by_email(email, conn);
    match user {
        Ok(usr) => {
            if let Some(usr) = usr {
                if let Some(tokens) = usr.tokens {

                    if let Some(_) = tokens.iter().position(|x| *x == token) {
                        return Ok(
                            model::Response{
                                success: true,
                                data: Some(true),
                                message: format!("OK"),
                                code: 200
                            }
                        );
                    }else{
                        return Ok(model::Response{
                                success: true,
                                data: Some(false),
                                message: format!("Unauthorized"),
                                code: 401
                            }
                        );
                    }
                } else{
                    return Ok(model::Response{
                                success: true,
                                data: Some(false),
                                message: format!("User has no tokens!"),
                                code: 401
                            }
                        );
                }
            } else {
                return Ok(model::Response{
                            success: false,
                            data: None,
                            message: format!("User not found!"),
                            code: 400
                        }
                    );
            }
        },
        Err(e) => return Err(e)
    }
}

*/
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


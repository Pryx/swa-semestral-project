use crate::db;
use crate::model;

use diesel::prelude::*;


pub trait UserServiceTrait {
    fn get_user_by_uid(&self, user_id: i32) -> Result<Option<model::User>, diesel::result::Error>; 
    fn get_user_by_email(&self, email: String) -> Result<Option<model::User>, diesel::result::Error>; 
    fn insert_token(&self, email: String, token: String) -> Result<Option<String>, diesel::result::Error>; 
    fn remove_token(&self, email: String, token: String) -> Result<bool, diesel::result::Error>; 
    fn register_user(&self, user: model::RegisterUser) -> Result<usize, diesel::result::Error>; 
    fn update_user(&self, email: String, user_data: model::RegisterUser) -> Result<usize, diesel::result::Error>; 
}
pub struct UserService{
    pub pool: std::sync::Arc<db::PgPool>
}

impl UserServiceTrait for UserService {
    fn get_user_by_uid(&self, user_id: i32) -> Result<Option<model::User>, diesel::result::Error>{
        use crate::schema::users::dsl::*;

        let conn = self.pool.get().expect("Error while connecting to db!");
        let user = users.
            filter(id.eq(user_id))
            .first::<model::User>(&conn)
            .optional()?;

        Ok(user)
    }
    fn get_user_by_email(&self, usr_email: String) -> Result<Option<model::User>, diesel::result::Error>{
        use crate::schema::users::dsl::*;

        let conn = self.pool.get().expect("Error while connecting to db!");
        let user = users.
            filter(email.eq(usr_email))
            .first::<model::User>(&conn)
            .optional()?;

        Ok(user)
    }
    fn insert_token(&self, email: String, token: String) -> Result<Option<String>, diesel::result::Error>{
        use crate::schema::users::dsl::users;
        use crate::schema::users::dsl::id;
        let conn = self.pool.get().expect("Error while connecting to db!");

        let usr = self.get_user_by_email(email);

        if let Err(e) = usr{
            return Err(e);
        }

        let usr = usr.unwrap();

        if let None = usr{
            return Ok(None);
        }

        let usr = usr.unwrap();

        let mut tokens_vec;
        if let Some(tokens) = usr.tokens {
            tokens_vec = tokens;
        } else{
            tokens_vec = vec!();
        }

        tokens_vec.push(token.clone());

        let res = 
            diesel::update(users.filter(id.eq(usr.id)))
            .set(crate::schema::users::tokens.eq(tokens_vec))
            .execute(&conn);

        match res {
            Ok(v) => {
                if v == 1{
                    return Ok(Some(token));
                }else{
                    return Ok(None)
                }
            }
            Err(e) => Err(e)
        }

    }

    fn remove_token(&self, email: String, token: String) -> Result<bool, diesel::result::Error>{
        use crate::schema::users::dsl::users;
        use crate::schema::users::dsl::id;
        
        let conn = self.pool.get().expect("Error while connecting to db!");

        let usr = self.get_user_by_email(email.clone());

        if let Err(e) = usr{
            return Err(e);
        }

        let usr = usr.unwrap();

        if let None = usr{
            return Ok(false);
        }

        let usr = usr.unwrap();

        let mut tokens = usr.tokens.unwrap();
        if let Some(pos) = tokens.iter().position(|x| *x == token) {
            tokens.remove(pos);
        } else {
            return Ok(false);
        }

        let res: Result<model::User, diesel::result::Error> = 
            diesel::update(users.filter(id.eq(usr.id)))
            .set(crate::schema::users::tokens.eq(tokens))
            .get_result(&conn);

        match res {
            Ok(_) => Ok(true),
            Err(e) => Err(e)
        }
    }

    fn register_user(&self, data: model::RegisterUser) -> Result<usize, diesel::result::Error>{
        use crate::schema::users::dsl::users;
        let conn = self.pool.get().expect("Error while connecting to db!");

        let rows_affected = diesel::insert_into(users)
            .values(data)
            .execute(&conn);

        return rows_affected;
    }

    fn update_user(&self, usr_email: String, user_data: model::RegisterUser) -> Result<usize, diesel::result::Error>{
        use crate::schema::users::dsl::users;
        use crate::schema::users::dsl::*;
        let conn = self.pool.get().expect("Error while connecting to db!");


        let rows_affected = diesel::update(users).filter(email.eq(usr_email.clone()))
            .set(
                (
                    email.eq(user_data.email.clone()),
                    firstname.eq(user_data.firstname),
                    lastname.eq(user_data.lastname),
                    pass.eq(user_data.pass)
                )
            )
            .execute(&conn);

        return rows_affected;
    }
}
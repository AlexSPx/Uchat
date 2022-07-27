use actix_session::Session;
use actix_web::{post, web::{Json, Data}, HttpResponse};
use log::error;
use serde_json::json;

use crate::{models::user::{UserCreate, register, login,UserCreationError, LoginUser}, DBPool};


#[post("/users")]
pub async fn create(user_data: Json<UserCreate>, pool: Data<DBPool>) -> HttpResponse {
    let conn = pool.get().expect("couldn't get DB connection from pool");
    let user_result = register(&conn, user_data);

    match user_result {
        Ok(user) => {
            HttpResponse::Created()
                    .content_type("application/json")
                    .json(json!(user))
        }
        Err(error) => {
            
            let error_message = match error {
                UserCreationError::DuplicatedEmail => "email is already taken",
                UserCreationError::DuplicatedUsername => "username is already taken",
            };
            
            HttpResponse::Conflict()
            .content_type("application/json")
            .body(error_message)
        }
    }

}

#[post("/login")]
pub async fn login_user(login_data: Json<LoginUser>, pool: Data<DBPool>, session: Session) -> HttpResponse {
    let conn = pool.get().expect("couldn't get DB connection from pool");
    let user_result = login(&conn, login_data);

    match user_result {
        Ok(user) => {
            session.insert("user_id", user.id).unwrap();

            HttpResponse::Ok()
                .content_type("application/json")
                .json(json!(user))
        },
        Err(err) => {
            HttpResponse::Unauthorized()
                .content_type("application/json")
                .json(err.to_string())
        },
    }      
}
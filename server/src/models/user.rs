use std::fmt::Display;

use actix_web::web::Json;
use scrypt::{Scrypt, password_hash::{SaltString, rand_core::OsRng, PasswordHash, PasswordVerifier, PasswordHasher}};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use diesel::{Insertable, Queryable, result::{Error, DatabaseErrorKind}, query_dsl::methods::FilterDsl, RunQueryDsl};

use crate::{schema::users, DBPooledConnection};

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "users"]
pub struct UserCreate {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub password: String,
}

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String
}

pub enum UserCreationError {
    DuplicatedEmail,
    DuplicatedUsername,
}

impl From<Error> for UserCreationError {
    fn from(err: Error) -> UserCreationError {
        if let Error::DatabaseError(DatabaseErrorKind::UniqueViolation, info) = &err {
            match info.constraint_name() {
                Some("users_username_key") => return UserCreationError::DuplicatedUsername,
                Some("users_email_key") => return UserCreationError::DuplicatedEmail,
                _ => {}
            }
        }
        panic!("Error creating user: {:?}", err)
    }
}

#[derive(Debug)]
pub enum UserLoginError {
    InvalidUsername,
    PasswordsDoesNotMatch
}

impl Display for UserLoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserLoginError::InvalidUsername => write!(f, "Invalid username"),
            UserLoginError::PasswordsDoesNotMatch => write!(f, "Incorect password"),
        }
    }
}

pub fn register(conn: &DBPooledConnection, incomming_data: Json<UserCreate>) -> Result<Vec<User>, UserCreationError> {
    use crate::schema::users::dsl::*;

    let mut user_data = incomming_data.0;

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Scrypt
        .hash_password(user_data.password.as_bytes(), &salt)
        .expect("Hash error")
        .to_string()
        .to_owned();

    user_data.password = hashed_password;

    diesel::insert_into(users)
        .values(user_data)
        .get_results::<User>(conn)
        .map_err(Into::into)
}

pub fn login(conn: &DBPooledConnection, login_data: Json<LoginUser>) -> Result<User, UserLoginError> {
    use crate::schema::users::dsl::*;
    use diesel::expression_methods::ExpressionMethods;

    let user = users
        .filter(username.eq(login_data.0.username))
        .get_result::<User>(conn)
        .map_err(|_| UserLoginError::InvalidUsername)?;

    let parsed_hash = PasswordHash::new(&user.password).unwrap();
    Scrypt
        .verify_password(&login_data.0.password.as_bytes(), &parsed_hash)
        .map_err(|_| UserLoginError::PasswordsDoesNotMatch)?;

    Ok(user)
}
#[macro_use] 
extern crate diesel;
extern crate dotenv;

use actix_session::{SessionMiddleware, storage::RedisSessionStore};
use diesel::{r2d2::ConnectionManager, PgConnection};
use dotenv::dotenv;


use actix_web::{App, HttpServer, web ,middleware::Logger, cookie::Key};
use r2d2::{PooledConnection, Pool};
use std::{io, env};

pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;
pub type DBPool = Pool<ConnectionManager<PgConnection>>;

mod models;
mod schema;
mod response;
mod routes;
mod constants;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    
    let secret = Key::generate();
    let redis_store = RedisSessionStore::new("redis://127.0.0.1:6379").await
        .expect("Redis connection error");

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");


    HttpServer::new(move || {
        App::new()
        .wrap(Logger::default())
        .wrap(SessionMiddleware::new(redis_store.clone(), secret.clone()))
        .app_data(web::Data::new(pool.clone()))
        .service(web::scope("/api")
            .service(routes::users::create)
            .service(routes::users::login_user)
        )   
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
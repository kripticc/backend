mod errors;
mod handlers;
mod models;
mod schema;

#[macro_use]
extern crate diesel;

use ::r2d2::Pool;
use actix_web::{dev::ServiceRequest, web, App, Error, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/user", web::get().to(handlers::get_users))
            .route("/user/{id}", web::get().to(handlers::get_user_by_id))
            .route("/user", web::post().to(handlers::add_user))
            .route("/user/{id}", web::delete().to(handlers::delete_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

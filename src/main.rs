#[macro_use]
extern crate diesel;

mod models;
mod routes;
mod services;

use crate::routes::home;
use crate::services::users;
use rocket::routes;

#[rocket::main]
async fn main() {
    let result = rocket::build()
        .mount(
            "/",
            routes![home::get_home, users::create_user, users::get_user],
        )
        .launch()
        .await;

    match result {
        Ok(result) => println!("app launched successfully {:?}", result),
        Err(error) => panic!("failed to launch with error {:?}", error),
    }
}

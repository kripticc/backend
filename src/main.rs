extern crate diesel;

mod routes;
mod services;
mod models;

use crate::routes::home;
use crate::services::users;
use rocket::routes;

#[rocket::main]
async fn main() {
    let result = rocket::build()
        .mount("/", routes![home::get_home, users::create_user])
        .launch()
        .await;

    match result {
        Ok(result) => println!("app launched successfully {:?}", result),
        Err(error) => panic!("failed to launch with error {:?}", error),
    }
}

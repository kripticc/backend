mod routes;

use rocket::routes;
use crate::routes::{test, home};

#[rocket::main]
async fn main() {
    let result = rocket::build()
        .mount("/hello", routes![test::get_hello_world])
        .mount("/", routes![home::get_home])
        .launch()
        .await;

    // Ok(result)
    match result {
        Ok(result) => println!("app launched successfully {:?}", result),
        Err(error) => panic!("failed to launch with error {:?}", error),
    }
}
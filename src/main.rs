mod routes;
use rocket::routes;
use crate::routes::{test::get_hello_world, home::get_home};

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/hello", routes![get_hello_world])
        .mount("/", routes![get_home])
        .launch()
        .await;
}
use rocket::get;

#[get("/world")]
pub fn get_hello_world() -> &'static str {
    "Hello, world!"
}
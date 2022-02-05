use rocket::get;

#[get("/users")]
pub fn create_user() -> &'static str {
    "trying to create a users..."
}

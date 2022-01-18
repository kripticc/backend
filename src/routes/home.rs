use rocket::get;

#[get("/")]
pub fn get_home() -> &'static str {
    "This is home."
}
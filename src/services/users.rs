use crate::models::user::insert_user;
// use backend_rust::establish_connection;
use rocket::post;
use rocket::serde::{json::Json, Deserialize};

#[derive(Deserialize)]
pub struct NewUser<'a> {
    pub email_hash: &'a str,
    pub password_hash: &'a str,
}
#[post("/users", data = "<user>")]
pub fn create_user(user: Json<NewUser>) -> String {
    // let connection = establish_connection();
    let new_user = insert_user(
        // &connection,
        user.email_hash,
        user.password_hash,
        "test/link/to/profile/pic",
        "Hi! I am a test user.",
        "hi i am new user.".to_string(),
    );
    serde_json::to_string(&new_user).unwrap()
}

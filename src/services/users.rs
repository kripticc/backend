use crate::models::user::User;
use rocket::post;
use rocket::serde::{json::Json, Deserialize};

#[derive(Deserialize)]
pub struct NewUser<String> {
    pub email_hash: String,
    pub password_hash: String,
}
#[post("/users", data = "<user>")]
pub fn create_user(user: Json<NewUser<String>>) -> String {
    let user = User {
        id: 0,
        email_hash: &*user.email_hash,
        password_hash: &*user.password_hash,
        user_name: "test_user_name",
        profile_pic: "test/link/to/profile/pic",
        bio: "Hi! I am a test user.",
        created_at: Default::default(),
        updated_at: Default::default(),
    };
    serde_json::to_string(&user).unwrap()
}

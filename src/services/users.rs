use crate::models::schema::users;
use backend_rust::establish_connection;
use chrono::{NaiveDateTime, Utc};
use diesel::RunQueryDsl;
use rocket::post;
use rocket::serde::{json::Json, Deserialize};
use scrypt::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Scrypt,
};

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct InputUserData {
    pub email_hash: String,
    pub password_hash: String,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub salt: String,
    pub email_hash: String,
    pub password_hash: String,
    pub profile_pic: String,
    pub bio: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[post("/users", format = "json", data = "<user>")]
pub fn create_user(user: Json<InputUserData>) -> Result<String, String> {
    let connection = establish_connection();
    let salt = SaltString::generate(&mut OsRng).as_str().parse().unwrap();
    // let password_hash = Scrypt
    //     .hash_password(user.password_hash.as_ref(), &salt)?
    //     .to_string();
    // let email_hash = Scrypt
    //     .hash_password(user.email_hash.as_ref(), &salt)?
    //     .to_string();
    let dt = Utc::now();
    let timestamp: i64 = dt.timestamp();
    let new_user = NewUser {
        name: "test_user_1".parse().unwrap(),
        salt,
        email_hash: user.email_hash.parse().unwrap(),
        password_hash: user.password_hash.parse().unwrap(),
        profile_pic: "path/to/pic".parse().unwrap(),
        bio: "hey this is my test account!".parse().unwrap(),
        created_at: NaiveDateTime::from_timestamp(timestamp, 0),
        updated_at: NaiveDateTime::from_timestamp(timestamp, 0),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&connection)
        .map_err(|_err| -> String { "Error when inserting".into() })
        .map(|_| "Successfully inserted!".into())
}

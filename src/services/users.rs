use crate::models::schema::users;
use backend_rust::establish_connection;
use chrono::{NaiveDateTime, Utc};
use diesel::{QueryDsl, RunQueryDsl};
use either::*;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, post};
use scrypt::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Scrypt,
};

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct UserRegisterInput {
    pub user_name: String,
    pub email_hash: String,
    pub password_hash: String,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct NewUser {
    pub user_name: String,
    pub salt: String,
    pub email_hash: String,
    pub password_hash: String,
    pub profile_pic: String,
    pub bio: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub salt: String,
    pub email_hash: String,
    pub password_hash: String,
    pub profile_pic: String,
    pub bio: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct Response {
    pub message: String,
}

#[post("/users", format = "json", data = "<user>")]
pub fn create_user(user: Json<UserRegisterInput>) -> Result<String, String> {
    let connection = establish_connection();
    let salt = SaltString::generate(&mut OsRng).as_str().parse().unwrap();
    let password_hash = Scrypt
        .hash_password(user.password_hash.as_ref(), &salt)
        .unwrap()
        .to_string();
    let email_hash = Scrypt
        .hash_password(user.email_hash.as_ref(), &salt)
        .unwrap()
        .to_string();
    let dt = Utc::now();
    let timestamp: i64 = dt.timestamp();
    let new_user = NewUser {
        user_name: user.user_name.parse().unwrap(),
        salt,
        email_hash,
        password_hash: (password_hash).parse().unwrap(),
        profile_pic: SaltString::generate(&mut OsRng).as_str().parse().unwrap(),
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

#[get("/user/<user_id>")]
pub fn get_user(user_id: i32) -> Either<Json<&'static str>, Json<User>> {
    let connection = establish_connection();
    let result = users::table.find(user_id).load::<User>(&connection);

    match result {
        Ok(mut result) => {
            return if result.len() == 0 {
                Left(Json("No user found"))
            } else {
                Right(Json(result.remove(0)))
            }
        }
        Err(error) => panic!("failed to launch with error {:?}", error),
    }
}

use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub bio: String,
    pub salt: String,
    pub profile_pic: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub user_name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub bio: &'a str,
    pub salt: &'a str,
    pub profile_pic: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

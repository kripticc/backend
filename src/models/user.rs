use crate::models::schema::users;
use chrono::{NaiveDateTime, Utc};
use diesel::PgConnection;
// use diesel::{PgConnection};
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User<'a> {
    pub id: Uuid,
    pub name: &'a str,
    pub email_hash: &'a str,
    pub password_hash: &'a str,
    pub profile_pic: &'a str,
    pub bio: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub fn insert_user<'a>(
    // conn: &PgConnection,
    name: &'a str,
    email_hash: &'a str,
    password_hash: &'a str,
    profile_pic: &'a str,
    bio: String,
) -> User<'a> {
    let dt = Utc::now();
    let timestamp: i64 = dt.timestamp();
    User {
        id: Uuid::new_v4(),
        name,
        email_hash,
        password_hash,
        profile_pic,
        bio,
        created_at: NaiveDateTime::from_timestamp(timestamp, 0),
        updated_at: NaiveDateTime::from_timestamp(timestamp, 0),
    }

    // diesel::insert_into(users::table)
    //     .values(&new_user)
    //     .get_result(conn)
    //     .expect("Error saving new post")
}

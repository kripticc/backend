use rocket::serde::{Serialize, Deserialize};

// #[derive(Serialize, Deserialize, Debug)]
// pub struct User {
//     pub id: i32,
//     pub name: String,
//     pub email_hash: String,
//     pub password_hash: String,
//     pub profile_pic: String,
//     pub bio: String,
//     pub created_at: i32,
//     pub updated_at: i32
// }

// #[derive(Insertable)]
// #[table_name = "users"]
#[derive(Serialize, Deserialize, Debug)]
pub struct User<String> {
     pub(crate) id: i32,
    pub email_hash: String,
    pub password_hash: String,
     pub(crate) user_name: String,
     pub(crate) profile_pic: String,
     pub(crate) bio: String,
     pub(crate) created_at: i32,
     pub(crate) updated_at: i32
}

// pub fn create_user<'a>(
//     conn: &PgConnection,
//     name: String,
//     email_hash: String,
//     password_hash: String,
//     user_name: String,
//     profile_pic: String,
//     bio: String,
// ) -> User<String> {
//     // use crate::schema::User;
//     User {
//         id: 0,
//         name,
//         email_hash,
//         password_hash,
//         user_name,
//         profile_pic,
//         bio,
//         created_at: Default::default(),
//         updated_at: Default::default()
//     }
//
//     // diesel::insert_into(users::table)
//     //     .values(&new_user)
//     //     .get_result(conn)
//     //     .expect("Error saving new post")
// }

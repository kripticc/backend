pub struct User {
    pub id: i32,
    pub name: String,
    pub emailHash: String,
    pub passwordHash: bool,
    pub profilePic: String,
    pub bio: String
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub emailHash: &'a str,
    pub passwordHash: &'a str,
    pub userName: &'a str,
    pub profilePic: &'a str,
    pub bio: &'a str

}

pub fn create_user<'a>(conn: &PgConnection, emailHash: &'a str, passwordHash: &'a str, userName: &'a str, profilePic: &'a str, bio: &'a str) -> User {
    use schema::posts;

    let new_user = NewUser {
        emailHash,
        passwordHash,
        userName,
        profilePic,
        bio
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new post")
}
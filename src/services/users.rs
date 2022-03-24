use crate::models;
use crate::models::schema::users;

fn insert_new_user(db: &PgConnection, user: CreateUser) -> Result<User, Error> {
    use self::schema::users::dsl::*;

    // Create insertion model
    let uuid = format!("{}", uuid::Uuid::new_v4());
    let new_user = models::NewUser {
        user_name: &user.name,
    };

    // normal diesel operations
    diesel::insert_into(users)
        .values(&new_user)
        .execute(&connection)
        .map_err(|_err| -> String { "Error when inserting".into() })
        .map(|_| "Successfully inserted!".into())
}

table! {
    users {
        id -> Uuid,
        name -> Varchar,
        salt -> Varchar,
        email_hash -> Varchar,
        password_hash -> Varchar,
        profile_pic -> Varchar,
        bio -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

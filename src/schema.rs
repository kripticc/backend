table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        author -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email_hash -> Varchar,
        password_hash -> Varchar,
        profile_pic -> Int4,
        bio -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(posts -> users (author));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);

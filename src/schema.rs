table! {
    users (id) {
        id -> Int4,
        user_name -> Text,
        email -> Text,
        password -> Text,
        bio -> Text,
        salt -> Text,
        profile_pic -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Timestamp,
    }
}

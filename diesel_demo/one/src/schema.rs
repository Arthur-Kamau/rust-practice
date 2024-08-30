// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        #[max_length = 150]
        title -> Varchar,
        body -> Text,
        published -> Bool,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 150]
        first_name -> Varchar,
        #[max_length = 150]
        last_name -> Varchar,
        #[max_length = 150]
        user_name -> Varchar,
        password -> Text,
        avatar_url -> Text,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);

// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Text,
        body -> Text,
        is_published -> Bool,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        password -> Text,
        avatar_url -> Text,
        is_blocked -> Bool,
        user_type -> Int4,
        reset_code -> Nullable<Int4>,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);

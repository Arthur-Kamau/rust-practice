// @generated automatically by Diesel CLI.

diesel::table! {
    comments (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        comment -> Text,
        parent -> Nullable<Int4>,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

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
        user_types -> Int4,
        #[max_length = 150]
        name -> Varchar,
        #[max_length = 150]
        email -> Varchar,
        bio -> Text,
        password -> Text,
        avatar_url -> Text,
        is_blocked -> Bool,
        blocked_reason -> Text,
        is_deleted -> Bool,
        reset_code -> Nullable<Int4>,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    comments,
    posts,
    users,
);

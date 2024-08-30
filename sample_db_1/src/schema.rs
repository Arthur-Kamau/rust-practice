// @generated automatically by Diesel CLI.

diesel::table! {
    departments (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Bpchar,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    employees (id) {
        id -> Int4,
        dept_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
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
        is_blocked -> Bool,
        blocked_reason -> Text,
        is_deleted -> Bool,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::joinable!(employees -> departments (dept_id));

diesel::allow_tables_to_appear_in_same_query!(
    departments,
    employees,
    users,
);

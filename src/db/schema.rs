// @generated automatically by Diesel CLI.

diesel::table! {
    HTML (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 10000]
        element -> Varchar,
    }
}

diesel::table! {
    expenses (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        cost -> Numeric,
        user_group_id -> Int4,
    }
}

diesel::table! {
    group (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        owner -> Int4,
        join_code -> Uuid,
    }
}

diesel::table! {
    user (id) {
        id -> Int4,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
    }
}

diesel::table! {
    user_group (user_group_id) {
        user_group_id -> Int4,
        user_id -> Int4,
        group_id -> Int4,
    }
}

diesel::joinable!(expenses -> user_group (user_group_id));
diesel::joinable!(group -> user (owner));
diesel::joinable!(user_group -> group (group_id));
diesel::joinable!(user_group -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    HTML,
    expenses,
    group,
    user,
    user_group,
);

// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 50]
        first_name -> Varchar,
        #[max_length = 50]
        last_name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
    }
}

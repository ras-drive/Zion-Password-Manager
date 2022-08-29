// @generated automatically by Diesel CLI.

diesel::table! {
    logins (id) {
        id -> Int4,
        email -> Varchar,
        password_hash -> Varchar,
    }
}

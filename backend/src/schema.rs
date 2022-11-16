// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Text,
        password_hash -> Text,
        salt -> Text,
    }
}

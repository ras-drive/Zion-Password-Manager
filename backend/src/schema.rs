// @generated automatically by Diesel CLI.

diesel::table! {
    session_cookies (cookie_id) {
        cookie_id -> Uuid,
        user_email -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Text,
        password_hash -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    session_cookies,
    users,
);

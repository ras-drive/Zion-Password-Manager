-- Your SQL goes here
CREATE TABLE session_cookies (
    cookie_id UUID PRIMARY KEY,
    user_email TEXT NOT NULL UNIQUE
)
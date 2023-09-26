-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    user_id UUID PRIMARY KEY,
    username TEXT NOT NULL,
    login TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    channels UUID[],
    Image TEXT,
    is_online BOOLEAN,
    last_online TIMESTAMP
);

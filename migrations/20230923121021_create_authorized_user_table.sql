-- Add migration script here
CREATE TABLE authorized_users (
    id UUID PRIMARY KEY,
    login VARCHAR(255) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    username VARCHAR(255) NOT NULL
);

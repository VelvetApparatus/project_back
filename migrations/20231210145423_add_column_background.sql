-- Add migration script here
-- Add 'background' column
ALTER TABLE users
ADD COLUMN background TEXT;
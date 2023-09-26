-- Add migration script here
CREATE TABLE IF NOT EXISTS channels (
    channel_id UUID PRIMARY KEY,
    last_message_id UUID,
    name TEXT NOT NULL,
    users UUID[],
    img TEXT
);
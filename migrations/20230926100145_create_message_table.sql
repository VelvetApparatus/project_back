-- Add migration script here
CREATE TABLE messages (
    message_id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    channel_id UUID NOT NULL,
    body TEXT NOT NULL,
    is_reply BOOLEAN,
    Image TEXT,
    created_at TIMESTAMP NOT NULL
);

-- Add foreign key constraints to reference the "User" and "Channel" tables
ALTER TABLE messages
ADD CONSTRAINT fk_user_id
FOREIGN KEY (user_id)
REFERENCES users (user_id);

ALTER TABLE messages
ADD CONSTRAINT fk_channel_id
FOREIGN KEY (channel_id)
REFERENCES channels (channel_id);
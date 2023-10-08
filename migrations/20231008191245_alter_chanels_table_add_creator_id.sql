-- Add migration script here
ALTER TABLE channels
ADD COLUMN creator_id UUID;

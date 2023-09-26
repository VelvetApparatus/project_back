-- Add migration script here
CREATE OR REPLACE FUNCTION get_user_channels(user_id UUID)
RETURNS TABLE (channel_name TEXT)
AS $$
BEGIN
    RETURN QUERY
    SELECT c.name
    FROM users AS u
    JOIN channels AS c ON u.channels @> ARRAY[c.id]
    WHERE u.user_id = user_id;
END;
$$ LANGUAGE plpgsql;
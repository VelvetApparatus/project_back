-- Add migration script here
-- Add migration script here
CREATE OR REPLACE FUNCTION get_channel_data(u_id UUID)
RETURNS TABLE (
    channel_id UUID,
    channel_name TEXT,
    last_message_id UUID,
    last_message_text TEXT,
    last_message_timestamp TIMESTAMP,
    channel_img TEXT
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        c.channel_id,
        c.name AS channel_name,
        c.last_message_id,
        m.body AS last_message_text,
        m.created_at AS last_message_timestamp,
        c.img AS channel_img
    FROM
        users u
    JOIN
        LATERAL unnest(u.channels) ch_id ON TRUE
    JOIN
        channels c ON c.channel_id = ch_id
    LEFT JOIN LATERAL (
        SELECT
            m.channel_id,
            m.message_id AS last_message_id,
            m.body,
            m.created_at
        FROM
            messages m
        WHERE
            m.channel_id = c.channel_id
        ORDER BY
            m.created_at DESC
        LIMIT 1
    ) m ON TRUE
    WHERE
        u.user_id = u_id;
END;
$$ LANGUAGE plpgsql;

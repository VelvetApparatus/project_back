CREATE OR REPLACE FUNCTION get_messages_by_channel(
    channel_id UUID,
    start_index INT,
    end_index INT
)
RETURNS TABLE (
    username TEXT,
    message_body TEXT,
    created_at TIMESTAMP
) AS $$
BEGIN
    RETURN QUERY (
        SELECT
            users.username,
            messages.body,
            messages.created_at
        FROM
            messages
        JOIN
            users ON messages.user_id = users.user_id
        WHERE
            messages.channel_id = channel_id
        ORDER BY
            messages.created_at
        OFFSET
            start_index
        LIMIT
            end_index - start_index + 1
    );
END;
$$ LANGUAGE plpgsql;

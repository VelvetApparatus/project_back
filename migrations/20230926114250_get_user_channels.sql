-- Add migration script here
CREATE OR REPLACE FUNCTION get_user_channels(user_id UUID)
RETURNS TABLE (
    channel_name TEXT,
    channel_img TEXT,
    message_body TEXT
) AS $$
DECLARE
    channel_id UUID;
BEGIN
    FOR channel_id IN SELECT UNNEST(channels) FROM users WHERE user_id = user_id LOOP
        SELECT
            channels.name,
            channels.img,
            messages.body
        INTO
            channel_name,
            channel_img,
            message_body
        FROM
            channels
        LEFT JOIN
            messages ON channels.channel_id = messages.channel_id
        WHERE
            channels.channel_id = channel_id
        ORDER BY
            messages.created_at DESC
        LIMIT 1;

        IF FOUND THEN
            RETURN NEXT;
        END IF;
    END LOOP;

    RETURN;
END;
$$ LANGUAGE plpgsql;

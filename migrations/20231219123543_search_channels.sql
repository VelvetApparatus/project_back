-- Add migration script here
CREATE OR REPLACE FUNCTION search_channels(search_text TEXT)
RETURNS TABLE (
    out_channel_id UUID,
    out_channel_name TEXT,
    out_last_message_id UUID,
    out_users UUID[],
    out_img TEXT
) AS $$
DECLARE
    channel_record RECORD;
BEGIN
    FOR channel_record IN SELECT channel_id, name, last_message_id, users, img
                         FROM channels WHERE name ILIKE '%' || search_text || '%'
    LOOP
        RETURN QUERY SELECT channel_record.channel_id,
                          channel_record.name,
                          channel_record.last_message_id,
                          channel_record.users,
                          channel_record.img;
    END LOOP;
END;
$$ LANGUAGE plpgsql;

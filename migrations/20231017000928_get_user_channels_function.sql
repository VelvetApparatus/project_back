-- Add migration script here
-- FUNCTION: public.get_user_channels(uuid)

-- DROP FUNCTION IF EXISTS public.get_user_channels(uuid);

CREATE OR REPLACE FUNCTION public.get_user_channels(
	u_id uuid)
    RETURNS TABLE(channel_name text, channel_img text, message_body text) 
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE PARALLEL UNSAFE
    ROWS 1000

AS $BODY$
DECLARE
    channel_id UUID;
BEGIN
    FOR channel_id IN SELECT UNNEST(channels) FROM users WHERE users.user_id = u_id LOOP
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
$BODY$;

ALTER FUNCTION public.get_user_channels(uuid)
    OWNER TO postgres;

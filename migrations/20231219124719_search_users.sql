-- Add migration script here
-- Add migration script here
CREATE OR REPLACE FUNCTION public.search_users(
	search_text text)
    RETURNS TABLE(out_user_id uuid, out_username text, out_icon text, out_background text, out_image text, out_is_online boolean, out_last_online timestamp without time zone) 
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE PARALLEL UNSAFE
    ROWS 1000

AS $BODY$
DECLARE
    user_record RECORD;
BEGIN
    FOR user_record IN SELECT user_id, username, image, is_online, last_online, icon, background
                      FROM users WHERE username ILIKE '%' || search_text || '%'
    LOOP
        RETURN QUERY SELECT user_record.user_id,
                          user_record.username,
                          user_record.icon,
                          user_record.background,
                          user_record.image,
                          user_record.is_online,
                          user_record.last_online;
    END LOOP;
END;
$BODY$;

ALTER FUNCTION public.search_users(text)
    OWNER TO postgres;

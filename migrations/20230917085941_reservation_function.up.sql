CREATE OR REPLACE FUNCTION rsvp.query(uid text, rid text, during TSTZRANGE) RETURNS TABLE (like rsvp.reservations)
AS $$
BEGIN
    if uid IS NULL AND rid IS NULL THEN
        RETURN QUERY SELECT * FROM rsvp.reservations WHERE timespan && during;
    elsif uid IS NULL THEN
        RETURN QUERY SELECT * FROM rsvp.reservations WHERE resource_id = rid AND timespan && during;
    elsif rid IS NULL THEN
        RETURN QUERY SELECT * FROM rsvp.reservations WHERE user_id = uid AND timespan && during;
    else
        RETURN QUERY SELECT * FROM rsvp.reservations WHERE user_id = uid AND resource_id = rid AND timespan && during;
    end if;
END;
$$ LANGUAGE plpgsql;

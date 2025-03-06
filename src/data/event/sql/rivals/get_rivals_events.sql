SELECT
    name,
    time,
    type AS 'event_type: EventType',
    gardeners,
    hours
FROM
    events
WHERE type = 'Rivals'
AND time BETWEEN $1 AND $2;

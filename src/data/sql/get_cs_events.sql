SELECT
    name,
    time,
    type AS 'event_type: EventType',
    gardeners,
    hours
FROM
    events
WHERE type = 'CS'
AND time BETWEEN $1 AND $2
AND gardeners = $3;

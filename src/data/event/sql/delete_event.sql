DELETE FROM events
WHERE name = $1 AND time = $2;

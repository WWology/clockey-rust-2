SELECT
    rank, id, score
FROM (
    SELECT
        DENSE_RANK() OVER (ORDER BY score DESC) rank,
        id,
        score
    FROM
        hok_scoreboard
)
WHERE
    id = $1;

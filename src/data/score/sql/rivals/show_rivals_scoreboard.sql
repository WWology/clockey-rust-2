SELECT
    DENSE_RANK() OVER (
        ORDER BY
            score DESC
    ) rank,
    id,
    score
FROM
    rivals_scoreboard;

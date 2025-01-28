SELECT
    DENSE_RANK() OVER (
        ORDER BY
            score DESC
    ) rank,
    id,
    score
FROM
    cs_scoreboard
ORDER BY
    score DESC;

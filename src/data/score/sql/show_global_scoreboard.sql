SELECT DENSE_RANK() OVER (
    ORDER BY
        sum(score) DESC
    ) rank,
    id,
    sum(score) score
from
(
    SELECT * FROM dota_scoreboard
    UNION ALL
    SELECT * FROM cs_scoreboard
    UNION ALL
    SELECT * FROM rivals_scoreboard
    UNION ALL
    SELECT * FROM mlbb_scoreboard
    UNION ALL
    SELECT * FROM hok_scoreboard
) as total_score
GROUP BY id;

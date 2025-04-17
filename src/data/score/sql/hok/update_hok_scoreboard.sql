INSERT INTO
    mlbb_scoreboard (id, score)
VALUES
    ($1, 1) ON CONFLICT (id) DO
UPDATE
SET
    score = score + 1;

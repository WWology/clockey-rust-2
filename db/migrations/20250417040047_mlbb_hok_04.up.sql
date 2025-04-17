-- Add up migration script here
CREATE TABLE
    IF NOT EXISTS mlbb_scoreboard (id INTEGER PRIMARY KEY, score INTEGER NOT NULL);

CREATE TABLE
    IF NOT EXISTS hok_scoreboard (id INTEGER PRIMARY KEY, score INTEGER NOT NULL);

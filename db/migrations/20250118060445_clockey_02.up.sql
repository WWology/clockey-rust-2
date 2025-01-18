-- Add up migration script here
CREATE TABLE
    IF NOT EXISTS dota_scoreboard (id INTEGER PRIMARY KEY, score INTEGER NOT NULL);

CREATE TABLE
    IF NOT EXISTS cs_scoreboard (id INTEGER PRIMARY KEY, score INTEGER NOT NULL);

-- Add up migration script here
CREATE TABLE
    IF NOT EXISTS rivals_scoreboard (id INTEGER PRIMARY KEY, score INTEGER NOT NULL);

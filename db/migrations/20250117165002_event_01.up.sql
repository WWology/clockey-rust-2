-- Add up migration script here
CREATE TABLE
    IF NOT EXISTS events (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        time INTEGER NOT NULL,
        type TEXT NOT NULL,
        gardeners INTEGER NOT NULL,
        hours INTEGER NOT NULL
    );

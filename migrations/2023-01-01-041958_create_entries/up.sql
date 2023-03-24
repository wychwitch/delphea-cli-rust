-- Your SQL goes here
CREATE TABLE entries (
    id INTEGER PRIMARY KEY NOT NULL,
    sheet_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    color TEXT NOT NULL,
    note TEXT NOT NULL,
    favorited BOOLEAN NOT NULL DEFAULT 0
)
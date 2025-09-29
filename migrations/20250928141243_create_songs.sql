-- Add migration script here
CREATE TABLE songs (
    id INTEGER PRIMARY KEY,
    artist TEXT NOT NULL,
    title TEXT NOT NULL
);

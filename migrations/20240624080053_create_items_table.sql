-- Add migration script here
CREATE TABLE IF NOT EXISTS items (
     id TEXT PRIMARY KEY,
     name TEXT NOT NULL,
     description TEXT NOT NULL
);

-- Your SQL goes here
CREATE TABLE users (
    username TEXT PRIMARY KEY NOT NULL,
    pass_hash TEXT NOT NULL,
    admin BOOLEAN NOT NULL
)
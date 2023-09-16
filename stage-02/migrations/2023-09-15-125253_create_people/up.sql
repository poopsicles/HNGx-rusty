-- Your SQL goes here
CREATE TABLE persons (
    id TEXT PRIMARY KEY NOT NULL, 
    name TEXT UNIQUE NOT NULL,
    age INTEGER NOT NULL CHECK (age > 0),
    favourite_colour INTEGER NOT NULL CHECK (favourite_colour BETWEEN 0 AND 7)
)
-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  passwd VARCHAR NOT NULL,
  email VARCHAR UNIQUE NOT NULL
)
-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE SPELL_BOOK (
  user_id UUID NOT NULL,
  name VARCHAR NOT NULL unique PRIMARY KEY,
  content VARCHAR NOT NULL,
  foreign key (user_id) references users(id)
)


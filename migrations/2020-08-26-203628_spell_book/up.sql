-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE SPELL_BOOK (
  id UUID NOT NULL PRIMARY KEY,
  name VARCHAR NOT NULL,
  content VARCHAR NOT NULL,
  foreign key (id) references users(id)
)


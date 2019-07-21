-- Your SQL goes here
CREATE TABLE heroes (
  id SERIAL PRIMARY KEY,
  name VARCHAR(60) NOT NULL,
  identity VARCHAR(60) NOT NULL,
  hometown VARCHAR(60) NOT NULL,
  age INTEGER NOT NULL
)

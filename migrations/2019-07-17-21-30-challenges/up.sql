-- Your SQL goes here
CREATE TABLE challenges (
  id SERIAL PRIMARY KEY,
  token VARCHAR(512) NOT NULL,
  challenge VARCHAR(512) NOT NULL,
  type_name VARCHAR(512) NOT NULL
)

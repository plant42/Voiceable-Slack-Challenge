-- Your SQL goes here

CREATE TABLE teams (
  id SERIAL PRIMARY KEY,
  team_id VARCHAR(64) NOT NULL,
  team_name VARCHAR(2048) NOT NULL,
  bot_id VARCHAR(2048) NOT NULL,
  bot_access_token VARCHAR(2048) NOT NULL
);

CREATE UNIQUE INDEX teams_team_id_idx ON teams (team_id);

CREATE TABLE access_tokens (
  id SERIAL PRIMARY KEY,
  access_token VARCHAR(512) NOT NULL,
  scope VARCHAR(512) NOT NULL,
  user_id VARCHAR(64) NOT NULL,
  team_id INTEGER REFERENCES teams(id)
);

CREATE UNIQUE INDEX access_tokens_user_id_idx ON access_tokens (user_id);

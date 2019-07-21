-- Your SQL goes here
CREATE TABLE authorizations (
                     id SERIAL PRIMARY KEY,
                     token VARCHAR(256) NOT NULL,
                     access_token_id INTEGER REFERENCES access_tokens (id)
);

CREATE INDEX authorization_access_token_id_idx ON authorizations(access_token_id);
CREATE UNIQUE INDEX authorization_token_idx ON authorizations(token);

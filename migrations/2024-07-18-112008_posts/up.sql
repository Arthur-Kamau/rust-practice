-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE posts (
                       id SERIAL PRIMARY KEY,
                       title  TEXT NOT NULL ,
                       body TEXT NOT NULL,
                       is_published BOOLEAN NOT NULL DEFAULT FALSE,
                       updated_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
                       created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
)
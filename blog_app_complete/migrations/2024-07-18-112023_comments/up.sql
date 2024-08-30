-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE comments (
                       id SERIAL PRIMARY KEY,
                       user_id integer ,
                       comment TEXT NOT NULL,
                       parent integer default null,
                       updated_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
                       created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
)
-- Your SQL goes here
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title  character varying(150) ,
    body TEXT NOT NULL,
     published BOOLEAN NOT NULL DEFAULT FALSE,
    updated_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
)
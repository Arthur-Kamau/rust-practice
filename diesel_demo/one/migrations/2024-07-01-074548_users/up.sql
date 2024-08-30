-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    first_name character varying(150) NOT NULL,
    last_name character varying(150) NOT NULL,
    user_name character varying(150) NOT NULL,
    password text NOT NULL,
    avatar_url TEXT NOT NULL,
    updated_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL

)
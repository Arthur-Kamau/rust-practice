-- Your SQL goes here
create table users (
                       id SERIAL PRIMARY KEY ,
                       user_types  integer  default 0  not null,
                       name character varying(150) not null,
                       email character varying(150) not null,
                       bio text not null,
                       password text not null,
                       avatar_url text not null,
                       is_blocked bool default false  not null,
                       blocked_reason text not null ,
                       is_deleted bool  default  false  not null,
                       reset_code integer,
                       updated_at  timestamp  without time zone default CURRENT_TIMESTAMP not null,
                       created_at  timestamp  without time zone default CURRENT_TIMESTAMP not null
);
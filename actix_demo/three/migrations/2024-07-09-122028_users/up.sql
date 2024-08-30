-- Your SQL goes here
create table users
(
    id         serial primary key,
    name       text                  not null,
    email      text                  not null,
    password   text                  not null,
    avatar_url   text                  not null,
    is_blocked boolean default false not null,
    user_type  integer default 0     not null,
    reset_code  integer  default null,
    updated_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    created_at timestamp without time zone default CURRENT_TIMESTAMP  not null
);
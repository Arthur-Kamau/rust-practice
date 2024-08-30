-- Your SQL goes here
create table posts
(
    id           serial primary key,
    title        text                  not null,
    body         text                  not null,
    is_published boolean default false not null,
    updated_at   timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    created_at   timestamp without time zone default CURRENT_TIMESTAMP  not null
)
-- Your SQL goes here
create table departments(
                            id SERIAL primary key ,
                            name character(100) not null,
                            updated_at  timestamp  without time zone default CURRENT_TIMESTAMP not null,
                            created_at  timestamp  without time zone default CURRENT_TIMESTAMP not null
);

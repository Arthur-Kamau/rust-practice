-- Your SQL goes here

create table employees(
                          id SERIAL primary key ,
                          dept_id int ,
                          user_id int ,
                          FOREIGN KEY   (dept_id)   REFERENCES departments(id),
                          updated_at  timestamp  without time zone default CURRENT_TIMESTAMP not null,
                          created_at  timestamp  without time zone default CURRENT_TIMESTAMP not null
)
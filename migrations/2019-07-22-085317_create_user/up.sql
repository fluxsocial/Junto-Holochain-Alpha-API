-- Your SQL goes here
CREATE TABLE users (
    id UUID PRIMARY KEY,
    email varchar,
    password varchar,
    pub_address varchar,
    first_name varchar,
    last_name varchar
)
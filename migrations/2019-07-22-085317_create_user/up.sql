-- Your SQL goes here
CREATE TABLE users (
    id UUID PRIMARY KEY,
    email text,
    password text,
    pub_address text,
    first_name text,
    last_name text
)
-- Your SQL goes here
CREATE TABLE users (
    id UUID PRIMARY KEY,
    email varchar,
    password varchar,
    agent_key varchar
)
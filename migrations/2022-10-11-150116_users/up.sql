-- Your SQL goes here
create table users(
    id uuid not null primary key default gen_random_uuid(),
    name varchar not null,
    email varchar unique not null,
    password varchar not null
);
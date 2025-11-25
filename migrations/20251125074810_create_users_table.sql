-- Add migration script here
create table users
(
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name varchar not null,
    email varchar not null unique,
    password varchar not null,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- table to do List
CREATE TABLE todos
(
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id uuid REFERENCES users(id) ON DELETE CASCADE,
    title varchar NOT NULL,
    description text,
    completed boolean NOT NULL DEFAULT false,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

create index idx_todos_user_id on todos(USER_ID);

-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users(
    user_id     UUID            PRIMARY KEY DEFAULT gen_random_uuid(),
    username    VARCHAR(256)    NOT NULL UNIQUE,
    password    VARCHAR(512)    NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS roles(
    role_id     UUID            PRIMARY KEY DEFAULT gen_random_uuid(),
    name        VARCHAR(256)    NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS user_roles(
    user_roles_id   UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID NOT NULL,
    role_id         UUID NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users,
    FOREIGN KEY (role_id) REFERENCES roles
);
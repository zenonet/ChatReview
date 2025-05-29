
DROP TABLE message_ratings;
DROP TABLE comments;
DROP TABLE chat_messages;
DROP TABLE chats;
DROP TABLE users;

CREATE TABLE users(
    id uuid PRIMARY KEY,
    username varchar(128) NOT NULL UNIQUE,
    password text NOT NULL,
    registration_date TIMESTAMPTZ NOT NULL
);

CREATE TABLE chats (
    id uuid PRIMARY KEY,
    name varchar(256),
    description text,
    user_id_a uuid REFERENCES users(id) NOT NULL,
    user_id_b uuid REFERENCES users(id)
);

CREATE TABLE chat_messages(
    id uuid PRIMARY KEY,
    content text NOT NULL,
    chat_id uuid REFERENCES chats(id) NOT NULL,
    is_owned_by_a boolean NOT NULL,
    index int NOT NULL
);

CREATE TABLE message_ratings(
    message_id uuid REFERENCES chat_messages(id) ON DELETE CASCADE NOT NULL,
    owner_id uuid REFERENCES users(id) ON DELETE CASCADE,
    value float NOT NULL,
    changed TIMESTAMPTZ NOT NULL,
    UNIQUE (message_id, owner_id)
);

CREATE TABLE comments(
    id uuid PRIMARY KEY,
    message_id uuid REFERENCES chat_messages(id) ON DELETE CASCADE NOT NULL,
    owner_id uuid REFERENCES users(id) NOT NULL,
    content text NOT NULL,
    time TIMESTAMPTZ
);


CREATE TABLE passkeys(
    id uuid PRIMARY KEY,
    user_id uuid REFERENCES users(id) NOT NULL,
    name TEXT NOT NULL,
    data TEXT NOT NULL,
    creation_date TIMESTAMPTZ NOT NULL
)

DROP TABLE message_ratings;
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
    owner_id uuid REFERENCES users(id) NOT NULL
);

CREATE TABLE chat_messages(
    id uuid PRIMARY KEY,
    content text NOT NULL,
    chat_id uuid REFERENCES chats(id) NOT NULL ON DELETE CASCADE,
    is_own boolean NOT NULL,
    index int NOT NULL
);

CREATE TABLE message_ratings(
    message_id uuid REFERENCES chat_messages(id) NOT NULL ON DELETE CASCADE,
    owner_id uuid REFERENCES users(id) ON DELETE CASCADE,
    value float NOT NULL,
    changed TIMESTAMPTZ NOT NULL,
    UNIQUE (message_id, owner_id)
);

CREATE TABLE comments(
    id uuid PRIMARY KEY,
    message_id uuid REFERENCES chat_messages(id) NOT NULL,
    owner_id uuid REFERENCES users(id) NOT NULL,
    content text NOT NULL,
    time TIMESTAMPTZ
);
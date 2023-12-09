CREATE TABLE users (
                       id INTEGER PRIMARY KEY AUTOINCREMENT,
                       name VARCHAR NOT NULL,
                       email VARCHAR UNIQUE NOT NULL,
                       password VARCHAR NOT NULL
);

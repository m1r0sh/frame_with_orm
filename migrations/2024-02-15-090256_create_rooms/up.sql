CREATE TABLE rooms (
                       id INTEGER PRIMARY KEY AUTOINCREMENT,
                       name VARCHAR NOT NULL,
                       user_id INTEGER REFERENCES users(id) ON DELETE CASCADE
);

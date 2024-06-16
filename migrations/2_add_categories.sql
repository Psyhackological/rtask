CREATE TABLE IF NOT EXISTS categories
(
    id   INTEGER PRIMARY KEY NOT NULL,
    name TEXT                NOT NULL UNIQUE
);

ALTER TABLE todos ADD COLUMN category_id INTEGER REFERENCES categories(id);

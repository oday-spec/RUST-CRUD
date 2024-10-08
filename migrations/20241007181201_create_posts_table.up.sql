-- Add up migration script here
CREATE TABLE IF NOT EXISTS posts (
    id INT NOT NULL,
    title VARCHAR(255) NOT NULL
);
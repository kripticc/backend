-- Your SQL goes here
CREATE TABLE posts (
                       id SERIAL PRIMARY KEY,
                       title VARCHAR NOT NULL,
                       body TEXT NOT NULL,
                       author INTEGER NOT NULL,
                       created_at TIMESTAMP NOT NULL,
                       updated_at TIMESTAMP NOT NULL,
                       CONSTRAINT fk_user
                           FOREIGN KEY(author)
                               REFERENCES users(id)
)
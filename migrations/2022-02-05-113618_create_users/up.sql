CREATE TABLE users (
                       id SERIAL PRIMARY KEY,
                       name VARCHAR NOT NULL,
                       email_hash VARCHAR NOT NULL,
                       password_hash VARCHAR NOT NULL,
                       profile_pic INTEGER NOT NULL,
                       bio TEXT,
                       created_at TIMESTAMP NOT NULL,
                       updated_at TIMESTAMP NOT NULL
)
CREATE TABLE users (
                       id VARCHAR PRIMARY KEY,
                       name VARCHAR NOT NULL,
                       salt VARCHAR NOT NULL,
                       email_hash VARCHAR NOT NULL,
                       password_hash VARCHAR NOT NULL,
                       profile_pic VARCHAR NOT NULL,
                       bio TEXT,
                       created_at TIMESTAMP NOT NULL,
                       updated_at TIMESTAMP NOT NULL
)
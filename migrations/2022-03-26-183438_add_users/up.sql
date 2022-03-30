CREATE TABLE users (
                       id SERIAL NOT NULL PRIMARY KEY,
                       user_name TEXT NOT NULL,
                       email TEXT NOT NULL,
                       password TEXT NOT NULL,
                       bio TEXT NOT NULL,
                       profile_pic TEXT NOT NULL,
                       created_at TIMESTAMP NOT NULL,
                       updated_at TIMESTAMP,
                       deleted_at TIMESTAMP
);
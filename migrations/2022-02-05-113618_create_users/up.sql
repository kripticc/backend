CREATE TABLE users (
                       id SERIAL PRIMARY KEY ,
                       user_name VARCHAR UNIQUE NOT NULL,
                       salt VARCHAR NOT NULL,
                       email_hash VARCHAR NOT NULL,
                       password_hash VARCHAR NOT NULL,
                       profile_pic VARCHAR UNIQUE NOT NULL,
                       bio TEXT,
                       created_at TIMESTAMP NOT NULL,
                       updated_at TIMESTAMP NOT NULL
)
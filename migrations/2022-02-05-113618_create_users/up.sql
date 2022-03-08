CREATE TABLE users (
                       user_name VARCHAR(255) PRIMARY KEY,
                       salt VARCHAR NOT NULL,
                       email_hash VARCHAR NOT NULL,
                       password_hash VARCHAR NOT NULL,
                       profile_pic VARCHAR UNIQUE NOT NULL,
                       bio TEXT,
                       created_at TIMESTAMP NOT NULL,
                       updated_at TIMESTAMP NOT NULL,
                       deleted_at TIMESTAMP NOT NULL
)
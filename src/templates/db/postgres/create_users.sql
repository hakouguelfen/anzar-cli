-- Add migration script here
CREATE TYPE role AS ENUM ('User', 'Admin');

CREATE TABLE users (
    id TEXT PRIMARY KEY DEFAULT gen_random_uuid()::TEXT,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    role TEXT NOT NULL,
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    "createdAt" TIMESTAMP DEFAULT NOW()
);

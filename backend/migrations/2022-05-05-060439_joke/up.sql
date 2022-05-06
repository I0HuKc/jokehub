CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS jokes_tb(
    uuid UUID PRIMARY KEY default uuid_generate_v4() NOT NULL,
    category VARCHAR(50) NOT NULL,
    language VARCHAR(4) NOT NULL,
    setup TEXT NOT NULL,
    punchline TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT now()
);
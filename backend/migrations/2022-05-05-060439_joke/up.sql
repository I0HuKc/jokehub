CREATE TABLE IF NOT EXISTS jokes_tb(
    uuid UUID PRIMARY KEY,
    category VARCHAR(50) NOT NULL,
    language VARCHAR(4) NOT NULL,
    setup TEXT NOT NULL,
    punchline TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT now()
);
-- Add migration script here
CREATE TABLE IF NOT EXISTS quote (
    quoteID UUID PRIMARY KEY,
    book varchar NOT NULL,
    quote TEXT not NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(book, quote)
);
-- Add migration script here
CREATE TABLE scrape_cache (
    id SERIAL PRIMARY KEY,
    question_url TEXT NOT NULL UNIQUE,
    input_format TEXT,
    constraints TEXT,
    statement TEXT,
    scraped_at TIMESTAMPTZ DEFAULT NOW(),
    ttl INTERVAL DEFAULT INTERVAL '7 days'
);

CREATE INDEX idx_scrape_cache_url ON scrape_cache (question_url);
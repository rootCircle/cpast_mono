-- Add migration script here
ALTER TABLE scrape_cache
ALTER COLUMN clex SET NOT NULL;

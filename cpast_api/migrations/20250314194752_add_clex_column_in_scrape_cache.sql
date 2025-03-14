-- Add migration script here
ALTER TABLE scrape_cache
ADD COLUMN clex TEXT;

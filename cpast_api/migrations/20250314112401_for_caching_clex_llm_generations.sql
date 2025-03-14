-- Add migration script here
CREATE TABLE llm_cache (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    input_hash TEXT UNIQUE NOT NULL,   -- Unique hash for input constraints
    constraints TEXT NOT NULL,         -- Store constraints separately
    input_format TEXT NOT NULL,        -- Store input format separately
    clex TEXT NOT NULL,                -- Store LLM output separately
    created_at TIMESTAMPTZ DEFAULT now(),  -- Timestamp of entry
    ttl INTERVAL DEFAULT '1 hour'      -- Time to live for cache entries
);

-- Index for fast lookup
CREATE INDEX idx_llm_cache_input_hash ON llm_cache(input_hash);

-- We might have to create a script to do routine cleanup of old entries
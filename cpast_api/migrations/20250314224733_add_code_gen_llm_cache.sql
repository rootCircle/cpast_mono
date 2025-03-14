-- Add migration script here
CREATE TABLE code_gen_llm_cache (
    id SERIAL PRIMARY KEY,
    question_url TEXT NOT NULL UNIQUE,
    code TEXT NOT NULL,
    generated_at TIMESTAMPTZ DEFAULT NOW(),
    ttl INTERVAL DEFAULT INTERVAL '1 days'
);

CREATE INDEX idx_code_gen_llm_cache_url ON code_gen_llm_cache (question_url);
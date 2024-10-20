-- Add share code table
CREATE TABLE shared_code (
    id SERIAL PRIMARY KEY,
    share_id VARCHAR(255) NOT NULL UNIQUE,
    code TEXT NOT NULL,
    code_language VARCHAR(255) NOT NULL,
    clex VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX shared_code_share_id_index ON shared_code (share_id);


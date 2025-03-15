-- Add migration script here
ALTER TABLE code_gen_llm_cache 
ALTER COLUMN language TYPE TEXT USING language::TEXT;


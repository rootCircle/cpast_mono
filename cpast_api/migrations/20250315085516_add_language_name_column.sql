-- Add migration script here
CREATE TYPE language_name AS ENUM ('Python', 'Cpp', 'C', 'Rust', 'Ruby', 'Javascript', 'Java');

ALTER TABLE code_gen_llm_cache 
ADD COLUMN language language_name DEFAULT 'Cpp';

UPDATE code_gen_llm_cache 
SET language = 'Cpp' 
WHERE language IS NULL;

ALTER TABLE code_gen_llm_cache 
ALTER COLUMN language SET NOT NULL,
ALTER COLUMN language DROP DEFAULT;


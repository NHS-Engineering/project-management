-- WARNING: do not revert this in production
ALTER TABLE tasks ADD COLUMN name_new TEXT NOT NULL CHECK (length(name_new) <= 32) DEFAULT "";
UPDATE tasks SET name_new = name;
ALTER TABLE tasks DROP COLUMN name;
ALTER TABLE tasks RENAME COLUMN name_new TO name;

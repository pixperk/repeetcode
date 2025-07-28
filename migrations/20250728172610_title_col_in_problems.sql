-- Add migration script here
ALTER TABLE tracked_problems
ADD COLUMN title TEXT NOT NULL;
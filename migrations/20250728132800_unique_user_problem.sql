-- Add migration script here
-- To avoid tracking same problem multiple times per user
ALTER TABLE tracked_problems ADD CONSTRAINT unique_user_problem UNIQUE(user_id, id);

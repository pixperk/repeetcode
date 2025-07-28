-- Add migration script here
ALTER TABLE tracked_problems
ADD CONSTRAINT unique_user_problem_slug UNIQUE (user_id, problem_slug);

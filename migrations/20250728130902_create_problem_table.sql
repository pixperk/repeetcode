-- Add migration script here
CREATE TABLE tracked_problems (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    problem_slug TEXT NOT NULL,
    stage INT NOT NULL DEFAULT 0,
    first_solve TIMESTAMPTZ NOT NULL,
    next_solve_on TIMESTAMPTZ NOT NULL,
    paused BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now()
);

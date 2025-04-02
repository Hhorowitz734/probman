-- Enable pgcrypto for UUID generation
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE problems (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    difficulty TEXT CHECK (difficulty IN ('Easy', 'Medium', 'Hard')) NOT NULL
);

CREATE TABLE test_cases (
    id SERIAL PRIMARY KEY,
    problem_id UUID REFERENCES problems(id) ON DELETE CASCADE,
    input TEXT NOT NULL,
    expected_output TEXT NOT NULL,
    visibility TEXT CHECK (visibility IN ('public', 'private')) NOT NULL
);

CREATE TABLE submissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    problem_id UUID REFERENCES problems(id) ON DELETE CASCADE,
    code TEXT NOT NULL,
    verdict TEXT CHECK (verdict IN ('Accepted', 'Wrong Answer', 'Runtime Error', 'Time Limit Exceeded', 'Compile Error')) NOT NULL,
    output TEXT,
    compile_error TEXT,
    submitted_at TIMESTAMPTZ DEFAULT now()
);

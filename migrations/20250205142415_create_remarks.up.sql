-- Add up migration script here

CREATE TABLE remarks (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    essence text NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
)

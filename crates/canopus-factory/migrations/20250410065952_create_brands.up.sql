CREATE TABLE brands (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    slug text NOT NULL,
    name text NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX brands_slug_index ON brands (slug);

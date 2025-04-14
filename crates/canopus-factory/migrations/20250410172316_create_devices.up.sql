CREATE TABLE devices (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    brand_id uuid NOT NULL references brands(id),
    name text NOT NULL,
    number_of_ports integer NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX devices_brand_id_index ON devices (brand_id);

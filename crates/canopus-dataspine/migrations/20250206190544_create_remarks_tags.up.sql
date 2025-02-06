-- Add up migration script here

CREATE TABLE remarks_tags (
    remark_id uuid NOT NULL,
    tag_id uuid NOT NULL
);

CREATE UNIQUE INDEX remark_id_tag_id_index ON remarks_tags (remark_id, tag_id);

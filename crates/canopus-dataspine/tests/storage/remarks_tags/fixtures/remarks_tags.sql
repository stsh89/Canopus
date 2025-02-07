INSERT INTO remarks (essence)
VALUES ('I think, therefore I am.');

WITH inserted_remark AS (
    INSERT INTO remarks (essence)
    VALUES ('The only source of knowledge is experience')
    RETURNING id
),
inserted_tag AS (
    INSERT INTO tags (title)
    VALUES ('Philosophy')
    RETURNING id
)
INSERT INTO remarks_tags (remark_id, tag_id)
SELECT inserted_remark.id, inserted_tag.id
FROM inserted_remark, inserted_tag;

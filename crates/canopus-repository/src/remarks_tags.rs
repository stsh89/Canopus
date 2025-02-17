use sqlx::PgTransaction;
use uuid::Uuid;

pub async fn create(
    tx: &mut PgTransaction<'_>,
    remark_id: Uuid,
    tag_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
INSERT INTO remarks_tags ( remark_id, tag_id )
VALUES ( $1, $2 )
        "#,
        remark_id,
        tag_id,
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn remark_tag_exists(tx: &mut PgTransaction<'_>, remark_id: Uuid, tag_id: Uuid) -> sqlx::Result<bool> {
    let exists = sqlx::query_scalar!(
        r#"
SELECT EXISTS(
    SELECT 1 FROM remarks_tags WHERE remark_id = $1 AND tag_id = $2
) as "exists!"
        "#,
        remark_id,
        tag_id
    ).fetch_one(&mut **tx)
    .await?;

    Ok(exists)
}

pub async fn delete_wasted_remarks_tags(tx: &mut PgTransaction<'_>) -> Result<u64, sqlx::Error> {
    let rec = sqlx::query!(
        r#"
WITH wasted_remarks_tags AS (
    SELECT remark_id
    FROM remarks_tags
    LEFT JOIN remarks ON remarks_tags.remark_id = remarks.id
    WHERE remarks.id IS NULL
)
DELETE FROM remarks_tags
WHERE remarks_tags.remark_id IN (SELECT wasted_remarks_tags.remark_id FROM wasted_remarks_tags)
        "#,
    )
    .execute(&mut **tx)
    .await?;

    Ok(rec.rows_affected())
}

pub async fn delete(
    tx: &mut PgTransaction<'_>,
    remark_id: Uuid,
    tag_id: Uuid,
) -> Result<u64, sqlx::Error> {
    let rec = sqlx::query!(
        r#"
DELETE FROM remarks_tags
WHERE remark_id = $1 AND tag_id = $2
        "#,
        remark_id,
        tag_id
    )
    .execute(&mut **tx)
    .await?;

    Ok(rec.rows_affected())
}

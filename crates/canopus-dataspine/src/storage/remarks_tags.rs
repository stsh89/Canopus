use sqlx::PgPool;
use uuid::Uuid;

pub async fn create(pool: &PgPool, remark_id: Uuid, tag_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
INSERT INTO remarks_tags ( remark_id, tag_id )
VALUES ( $1, $2 )
        "#,
        remark_id,
        tag_id,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete(pool: &PgPool, remark_id: Uuid, tag_id: Uuid) -> Result<u64, sqlx::Error> {
    let rec = sqlx::query!(
        r#"
DELETE FROM remarks_tags
WHERE remark_id = $1 AND tag_id = $2
        "#,
        remark_id,
        tag_id
    )
    .execute(pool)
    .await?;

    Ok(rec.rows_affected())
}

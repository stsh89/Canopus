use sqlx::{PgPool, PgTransaction};
use uuid::Uuid;

pub async fn create(tx: &mut PgTransaction<'_>, title: &str) -> Result<Uuid, sqlx::Error> {
    let rec = sqlx::query!(
        r#"
INSERT INTO tags ( title )
VALUES ( lower($1) )
RETURNING id
        "#,
        title
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(rec.id)
}

pub async fn delete(pool: &PgPool, id: Uuid) -> Result<u64, sqlx::Error> {
    let rec = sqlx::query!("DELETE FROM tags WHERE id = $1", id)
        .execute(pool)
        .await?;

    Ok(rec.rows_affected())
}

pub async fn get(pool: &PgPool, id: Uuid) -> Result<String, sqlx::Error> {
    let rec = sqlx::query!("SELECT title FROM tags WHERE id = $1", id)
        .fetch_one(pool)
        .await?;

    Ok(rec.title)
}

pub async fn find(tx: &mut PgTransaction<'_>, title: &str) -> Result<Option<Uuid>, sqlx::Error> {
    let rec = sqlx::query_scalar!(r#"SELECT id FROM tags WHERE title = $1"#, title)
        .fetch_optional(&mut **tx)
        .await?;

    Ok(rec)
}

pub async fn update(pool: &PgPool, id: Uuid, title: &str) -> Result<u64, sqlx::Error> {
    let rec = sqlx::query!(
        r#"
UPDATE tags
SET
    title = lower($2),
    updated_at = DEFAULT
WHERE
    id = $1
        "#,
        id,
        title
    )
    .execute(pool)
    .await?;

    Ok(rec.rows_affected())
}

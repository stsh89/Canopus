use sqlx::PgPool;
use uuid::Uuid;

pub async fn create(pool: &PgPool, essence: &str) -> Result<Uuid, sqlx::Error> {
    let rec = sqlx::query!(
        r#"
INSERT INTO remarks ( essence )
VALUES ( $1 )
RETURNING id
        "#,
        essence
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
}

pub async fn delete(pool: &PgPool, id: Uuid) -> Result<u64, sqlx::Error> {
    let rec = sqlx::query!("DELETE FROM remarks WHERE id = $1", id)
        .execute(pool)
        .await?;

    Ok(rec.rows_affected())
}

pub async fn get(pool: &PgPool, id: Uuid) -> Result<String, sqlx::Error> {
    let rec = sqlx::query!("SELECT essence FROM remarks WHERE id = $1", id)
        .fetch_one(pool)
        .await?;

    Ok(rec.essence)
}

pub async fn update(pool: &PgPool, id: Uuid, essence: &str) -> Result<u64, sqlx::Error> {
    let rec = sqlx::query!(
        r#"
UPDATE remarks
SET
    essence = $2,
    updated_at = DEFAULT
WHERE
    id = $1
        "#,
        id,
        essence
    )
    .execute(pool)
    .await?;

    Ok(rec.rows_affected())
}

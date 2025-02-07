use sqlx::PgPool;
use uuid::Uuid;

pub async fn create(pool: &PgPool, title: &str) -> Result<Uuid, sqlx::Error> {
    let rec = sqlx::query!(
        r#"
INSERT INTO tags ( title )
VALUES ( $1 )
RETURNING id
        "#,
        title
    )
    .fetch_one(pool)
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

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

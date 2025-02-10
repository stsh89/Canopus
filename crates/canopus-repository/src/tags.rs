use canopus_protocol::tags::{Tag, TagAttributes};
use chrono::{DateTime, Utc};
use sqlx::{PgPool, PgTransaction};
use uuid::Uuid;

pub struct TagRow {
    pub id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn create_tag(tx: &mut PgTransaction<'_>, title: &str) -> Result<Uuid, sqlx::Error> {
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

pub async fn delete_tag(pool: &PgPool, id: Uuid) -> Result<u64, sqlx::Error> {
    let rec = sqlx::query!("DELETE FROM tags WHERE id = $1", id)
        .execute(pool)
        .await?;

    Ok(rec.rows_affected())
}

pub async fn get_tag(pool: &PgPool, id: Uuid) -> Result<TagRow, sqlx::Error> {
    let row = sqlx::query_as!(TagRow, "SELECT * FROM tags WHERE id = $1", id)
        .fetch_one(pool)
        .await?;

    Ok(row)
}

pub async fn find(tx: &mut PgTransaction<'_>, title: &str) -> Result<Option<Uuid>, sqlx::Error> {
    let rec = sqlx::query_scalar!(r#"SELECT id FROM tags WHERE title = lower($1)"#, title)
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

impl From<TagRow> for Tag {
    fn from(value: TagRow) -> Self {
        let TagRow {
            id,
            title,
            created_at,
            updated_at,
        } = value;

        Self::new(TagAttributes {
            id,
            title,
            created_at,
            updated_at,
        })
    }
}

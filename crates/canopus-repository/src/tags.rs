use canopus_protocol::tags::{Tag, TagAttributes};
use chrono::{DateTime, Utc};
use sqlx::{PgPool, PgTransaction};
use uuid::Uuid;

use crate::{DEFAULT_PAGE_SIZE, PaginationToken};

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

pub async fn delete_wasted_tags(tx: &mut PgTransaction<'_>) -> Result<u64, sqlx::Error> {
    let rec = sqlx::query!(
        r#"
WITH wasted_tags AS (
    SELECT id
    FROM tags
    LEFT JOIN remarks_tags ON remarks_tags.tag_id = tags.id
    WHERE remarks_tags.tag_id IS NULL
)
DELETE FROM tags
WHERE tags.id IN (SELECT wasted_tags.id FROM wasted_tags)
        "#
    )
    .execute(&mut **tx)
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

pub async fn list_tags(
    pool: &PgPool,
    pagination_token: Option<PaginationToken>,
) -> sqlx::Result<Vec<TagRow>> {
    let pagination_id = pagination_token
        .as_ref()
        .map(|token| token.id)
        .unwrap_or(Uuid::nil());

    let pagination_created_at = pagination_token
        .map(|token| token.created_at)
        .unwrap_or(Utc::now());

    let rows = sqlx::query_as!(
        TagRow,
        r#"
            SELECT * FROM tags
            WHERE
                created_at < $1 OR (created_at = $1 AND id > $2)
            ORDER BY created_at DESC, id ASC
            LIMIT $3
        "#,
        pagination_created_at,
        pagination_id,
        DEFAULT_PAGE_SIZE,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
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

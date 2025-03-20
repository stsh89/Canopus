use crate::{DEFAULT_PAGE_SIZE, PaginationToken, tags::TagRow};
use canopus_definitions::{Remark, RemarkAttributes};
use chrono::{DateTime, Utc};
use sqlx::{PgPool, PgTransaction};
use uuid::Uuid;

pub struct RemarkRow {
    pub id: Uuid,
    pub essence: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn create_remark(tx: &mut PgTransaction<'_>, essence: &str) -> sqlx::Result<Uuid> {
    let rec = sqlx::query!(
        r#"
INSERT INTO remarks ( essence )
VALUES ( $1 )
RETURNING id
        "#,
        essence
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(rec.id)
}

pub async fn delete_remark(
    tx: &mut PgTransaction<'_>,
    remark_id: Uuid,
) -> Result<u64, sqlx::Error> {
    let rec = sqlx::query!("DELETE FROM remarks WHERE id = $1", remark_id)
        .execute(&mut **tx)
        .await?;

    Ok(rec.rows_affected())
}

pub async fn get_remark(pool: &PgPool, remark_id: Uuid) -> Result<RemarkRow, sqlx::Error> {
    let row = sqlx::query_as!(RemarkRow, "SELECT * FROM remarks WHERE id = $1", remark_id)
        .fetch_one(pool)
        .await?;

    Ok(row)
}

pub async fn list_remarks(
    pool: &PgPool,
    pagination_token: Option<PaginationToken>,
) -> sqlx::Result<Vec<RemarkRow>> {
    let pagination_id = pagination_token
        .as_ref()
        .map(|token| token.id)
        .unwrap_or(Uuid::nil());
    let pagination_created_at = pagination_token
        .map(|token| token.created_at)
        .unwrap_or(Utc::now());

    let rows = sqlx::query_as!(
        RemarkRow,
        r#"
            SELECT * FROM remarks
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

pub async fn list_tags(pool: &PgPool, remark_id: Uuid) -> sqlx::Result<Vec<TagRow>> {
    sqlx::query_as!(
        TagRow,
        r#"
SELECT tags.* FROM tags
JOIN remarks_tags ON remarks_tags.tag_id = tags.id AND remarks_tags.remark_id = $1
ORDER BY tags.title DESC
        "#,
        remark_id
    )
    .fetch_all(pool)
    .await
}

pub async fn preload_tags(pool: &PgPool, remark_ids: &[Uuid]) -> sqlx::Result<Vec<(Uuid, TagRow)>> {
    use sqlx::Row;

    let rows = sqlx::query(
        r#"
SELECT remarks_tags.remark_id, tags.*
FROM tags
JOIN remarks_tags ON remarks_tags.tag_id = tags.id
WHERE remarks_tags.remark_id = ANY($1)
ORDER BY tags.title DESC
        "#,
    )
    .bind(remark_ids)
    .fetch_all(pool)
    .await?;

    let tags = rows
        .into_iter()
        .map(|row| {
            (
                row.get("remark_id"),
                TagRow {
                    id: row.get("id"),
                    title: row.get("title"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                },
            )
        })
        .collect();

    Ok(tags)
}

pub async fn update_remark(
    tx: &mut PgTransaction<'_>,
    id: Uuid,
    essence: &str,
) -> Result<u64, sqlx::Error> {
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
    .execute(&mut **tx)
    .await?;

    Ok(rec.rows_affected())
}

impl From<RemarkRow> for Remark {
    fn from(value: RemarkRow) -> Self {
        let RemarkRow {
            id,
            essence,
            created_at,
            updated_at,
        } = value;

        Self::new(RemarkAttributes {
            id,
            essence,
            tags: vec![],
            created_at,
            updated_at,
        })
    }
}

use canopus_protocol::remarks::{Remark, RemarkAttributes};
use chrono::{DateTime, Utc};
use sqlx::{PgPool, PgTransaction};
use uuid::Uuid;

use crate::tags::TagRow;

pub struct RemarkRow {
    pub id: Uuid,
    pub essence: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn create_remark(tx: &mut PgTransaction<'_>, essence: &str) -> Result<Uuid, sqlx::Error> {
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

pub async fn list_tags(pool: &PgPool, remark_id: Uuid) -> sqlx::Result<Vec<TagRow>> {
    let rows = sqlx::query_as!(
        TagRow,
        r#"
            SELECT tags.* FROM tags
            JOIN remarks_tags ON remarks_tags.tag_id = tags.id AND remarks_tags.remark_id = $1
            ORDER BY tags.created_at DESC
        "#,
        remark_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
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

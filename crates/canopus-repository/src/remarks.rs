use crate::{
    DEFAULT_PAGE_SIZE, Repository, TagRow, TagTitleRow, URL_SAFE_NO_PAD_ENGINE, commit_transaction,
    from_sqlx_err, remarks_tags,
};
use canopus_definitions::{
    ApplicationError, ApplicationResult, Page, Remark, RemarkAttributes, RemarkEssence, Tag,
    TagAttributes, TagTitle,
};
use canopus_operations::remarks::{
    DeleteRemark, GetRemark, InsertRemark, ListRemarks, NewRemark, RemarksPageParameters,
    UpdateRemark,
};
use chrono::{DateTime, Utc};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, PgTransaction};
use std::{collections::HashMap, str::FromStr};
use uuid::Uuid;

pub struct RemarkRow {
    pub id: Uuid,
    pub essence: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl DeleteRemark for Repository {
    #[tracing::instrument(skip_all)]
    async fn delete_remark(&self, remark: &Remark) -> ApplicationResult<()> {
        let mut tx = self.begin_transaction().await?;

        let rec = sqlx::query!("DELETE FROM remarks WHERE id = $1", remark.id())
            .execute(&mut *tx)
            .await
            .map_err(from_sqlx_err)?;

        if rec.rows_affected() == 0 {
            return Err(ApplicationError::NotFound);
        }

        remarks_tags::delete_unused_remarks_tags(&mut tx).await?;
        delete_unused_tags(&mut tx).await?;

        commit_transaction(tx).await
    }
}

impl GetRemark for Repository {
    #[tracing::instrument(skip_all)]
    async fn get_remark(&self, id: Uuid) -> ApplicationResult<Remark> {
        let RemarkRow {
            id,
            essence,
            created_at,
            updated_at,
        } = sqlx::query_as!(RemarkRow, "SELECT * FROM remarks WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await
            .map_err(from_sqlx_err)?;

        let tag_titles = sqlx::query_as!(
            TagTitleRow,
            r#"
SELECT tags.title FROM tags
JOIN remarks_tags ON remarks_tags.tag_id = tags.id AND remarks_tags.remark_id = $1
ORDER BY tags.title DESC
            "#,
            id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(from_sqlx_err)?;

        let tags = tag_titles
            .into_iter()
            .map(TryInto::<TagTitle>::try_into)
            .collect::<ApplicationResult<Vec<TagTitle>>>()?;

        let remark = Remark::new(RemarkAttributes {
            id,
            essence: RemarkEssence::new(essence)?,
            tags,
            created_at,
            updated_at,
        });

        Ok(remark)
    }
}

impl InsertRemark for Repository {
    #[tracing::instrument(skip_all)]
    async fn insert_remark(&self, new_remark: NewRemark) -> Result<Remark, ApplicationError> {
        let NewRemark { essence, tags } = new_remark;

        let mut tx = self.begin_transaction().await?;

        let rec = sqlx::query!(
            r#"
INSERT INTO remarks ( essence )
VALUES ( $1 )
RETURNING id, created_at, updated_at
            "#,
            essence.as_str()
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(from_sqlx_err)?;

        link_tags(&mut tx, rec.id, tags.iter().collect()).await?;

        commit_transaction(tx).await?;

        let remark = Remark::new(RemarkAttributes {
            id: rec.id,
            essence,
            tags,
            created_at: rec.created_at,
            updated_at: rec.updated_at,
        });

        Ok(remark)
    }
}

impl ListRemarks for Repository {
    #[tracing::instrument(skip_all)]
    async fn list_remarks(
        &self,
        parameters: RemarksPageParameters,
    ) -> ApplicationResult<Page<Remark>> {
        let RemarksPageParameters { page_token } = parameters;

        let page_token = page_token.map(TryInto::<PageToken>::try_into).transpose()?;

        let last_id = page_token
            .as_ref()
            .map(|token| token.id)
            .unwrap_or(Uuid::nil());

        let last_created_at = page_token
            .map(|token| token.created_at)
            .unwrap_or(Utc::now());

        let rows = sqlx::query_as!(
            RemarkRow,
            r#"
SELECT * FROM remarks
WHERE created_at < $1 OR (created_at = $1 AND id > $2)
ORDER BY created_at DESC, id ASC
LIMIT $3
            "#,
            last_created_at,
            last_id,
            DEFAULT_PAGE_SIZE,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(from_sqlx_err)?;

        let next_page_token = PageToken::from_rows(&rows).map(Into::into);
        let remark_ids: Vec<Uuid> = rows.iter().map(|row| row.id).collect();
        let remarks_tags = preload_tags(&self.pool, &remark_ids)
            .await
            .map_err(from_sqlx_err)?;
        let mut grouped_tags: HashMap<Uuid, Vec<TagRow>> =
            remarks_tags.into_iter().into_group_map();

        let items: Vec<Remark> = rows
            .into_iter()
            .map(|row| {
                let RemarkRow {
                    id,
                    essence,
                    created_at,
                    updated_at,
                } = row;

                let tags = grouped_tags
                    .remove(&row.id)
                    .unwrap_or_default()
                    .into_iter()
                    .map(|row| TagTitle::new(row.title))
                    .collect::<ApplicationResult<Vec<TagTitle>>>()?;

                let remark = Remark::new(RemarkAttributes {
                    id,
                    essence: RemarkEssence::new(essence)?,
                    tags,
                    created_at,
                    updated_at,
                });

                Ok(remark)
            })
            .collect::<ApplicationResult<Vec<Remark>>>()?;

        Ok(Page {
            next_page_token,
            items,
        })
    }
}

impl UpdateRemark for Repository {
    #[tracing::instrument(skip_all)]
    async fn update_remark(&self, remark: &mut Remark) -> Result<(), ApplicationError> {
        let mut tx = self.begin_transaction().await?;

        let rec = sqlx::query!(
            r#"
UPDATE remarks
SET essence = $2, updated_at = DEFAULT
WHERE id = $1
RETURNING updated_at
            "#,
            remark.id(),
            remark.essence().as_str()
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(from_sqlx_err)?;

        remark.set_updated_at(rec.updated_at)?;

        unlink_tags(&mut tx, remark.id()).await?;
        link_tags(&mut tx, remark.id(), remark.tags()).await?;
        delete_unused_tags(&mut tx).await?;

        commit_transaction(tx).await?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
struct PageToken {
    id: Uuid,
    created_at: DateTime<Utc>,
}

impl PageToken {
    fn from_rows(rows: &[RemarkRow]) -> Option<Self> {
        if rows.len() < DEFAULT_PAGE_SIZE as usize {
            return None;
        }

        rows.last().map(|row| PageToken {
            id: row.id,
            created_at: row.created_at,
        })
    }
}

impl FromStr for PageToken {
    type Err = eyre::Error;

    fn from_str(s: &str) -> eyre::Result<Self> {
        use base64::Engine;

        let json = URL_SAFE_NO_PAD_ENGINE.decode(s)?;
        let token = serde_json::from_slice(&json)?;

        Ok(token)
    }
}

impl TryFrom<canopus_definitions::PageToken> for PageToken {
    type Error = ApplicationError;

    fn try_from(value: canopus_definitions::PageToken) -> ApplicationResult<Self> {
        value
            .parse()
            .map_err(|_err| ApplicationError::invalid_argument("malformed tags page token"))
    }
}

async fn insert_tag(tx: &mut PgTransaction<'_>, title: TagTitle) -> ApplicationResult<Tag> {
    let rec = sqlx::query!(
        r#"
INSERT INTO tags ( title )
VALUES ( $1 )
RETURNING id, created_at, updated_at
        "#,
        title.as_str()
    )
    .fetch_one(&mut **tx)
    .await
    .map_err(from_sqlx_err)?;

    let tag = Tag::new(TagAttributes {
        id: rec.id,
        title,
        created_at: rec.created_at,
        updated_at: rec.updated_at,
    });

    Ok(tag)
}

async fn link_tags(
    tx: &mut PgTransaction<'_>,
    remark_id: Uuid,
    tags: Vec<&TagTitle>,
) -> ApplicationResult<()> {
    for title in tags {
        let tag = find_or_create_tag(tx, title.clone()).await?;
        assign_remark_tags(tx, remark_id, &tag).await?;
    }

    Ok(())
}

async fn assign_remark_tags(
    tx: &mut PgTransaction<'_>,
    remark_id: Uuid,
    tag: &Tag,
) -> ApplicationResult<()> {
    if !remarks_tags::remark_tag_exists(tx, remark_id, tag.id())
        .await
        .map_err(from_sqlx_err)?
    {
        remarks_tags::create(tx, remark_id, tag.id())
            .await
            .map_err(from_sqlx_err)?;
    }

    Ok(())
}

async fn find_or_create_tag(tx: &mut PgTransaction<'_>, title: TagTitle) -> ApplicationResult<Tag> {
    let Some(tag) = find_tag_by_title(tx, &title).await? else {
        return insert_tag(tx, title).await;
    };

    Ok(tag)
}

pub async fn delete_unused_tags(tx: &mut PgTransaction<'_>) -> ApplicationResult<()> {
    sqlx::query!(
        r#"
WITH unused_tags AS (
    SELECT id
    FROM tags
    LEFT JOIN remarks_tags ON remarks_tags.tag_id = tags.id
    WHERE remarks_tags.tag_id IS NULL
)
DELETE FROM tags
WHERE tags.id IN (SELECT unused_tags.id FROM unused_tags)
        "#
    )
    .execute(&mut **tx)
    .await
    .map_err(from_sqlx_err)?;

    Ok(())
}

async fn find_tag_by_title(
    tx: &mut PgTransaction<'_>,
    title: &TagTitle,
) -> ApplicationResult<Option<Tag>> {
    let tag = sqlx::query_as!(
        TagRow,
        r#"SELECT * FROM tags WHERE title = $1"#,
        title.as_str()
    )
    .fetch_optional(&mut **tx)
    .await
    .map_err(from_sqlx_err)?;

    tag.map(TryInto::try_into).transpose()
}

async fn preload_tags(pool: &PgPool, remark_ids: &[Uuid]) -> sqlx::Result<Vec<(Uuid, TagRow)>> {
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

async fn unlink_tags(tx: &mut PgTransaction<'_>, remark_id: Uuid) -> ApplicationResult<()> {
    sqlx::query!(
        r#"DELETE FROM remarks_tags WHERE remark_id = $1"#,
        remark_id
    )
    .execute(&mut **tx)
    .await
    .map_err(from_sqlx_err)?;

    Ok(())
}

impl std::fmt::Display for PageToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use base64::Engine;

        let json = serde_json::to_string(&self).map_err(|_err| std::fmt::Error)?;

        let encoded_json = URL_SAFE_NO_PAD_ENGINE.encode(json);

        f.write_str(&encoded_json)
    }
}

impl From<PageToken> for canopus_definitions::PageToken {
    fn from(value: PageToken) -> Self {
        value.to_string().into()
    }
}

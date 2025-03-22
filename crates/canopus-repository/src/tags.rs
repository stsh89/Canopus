use crate::{
    DEFAULT_PAGE_SIZE, Repository, URL_SAFE_NO_PAD_ENGINE, commit_transaction, from_sqlx_err,
};
use canopus_definitions::{
    ApplicationError, ApplicationResult, Page, Tag, TagAttributes, TagTitle,
};
use canopus_operations::tags::{GetTag, ListTags, TagsPageParameters, UpdateTag};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

pub struct TagRow {
    pub id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct TagTitleRow {
    pub title: String,
}

impl GetTag for Repository {
    #[tracing::instrument(skip_all)]
    async fn get_tag(&self, id: Uuid) -> ApplicationResult<Tag> {
        sqlx::query_as!(TagRow, "SELECT * FROM tags WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await
            .map_err(from_sqlx_err)?
            .try_into()
    }
}

impl ListTags for Repository {
    #[tracing::instrument(skip_all)]
    async fn list_tags(&self, parameters: TagsPageParameters) -> ApplicationResult<Page<Tag>> {
        let TagsPageParameters { page_token } = parameters;

        let page_token = page_token.map(TryInto::<PageToken>::try_into).transpose()?;

        let last_id = page_token
            .as_ref()
            .map(|token| token.id)
            .unwrap_or(Uuid::nil());

        let last_created_at = page_token
            .map(|token| token.created_at)
            .unwrap_or(Utc::now());

        let rows = sqlx::query_as!(
            TagRow,
            r#"
SELECT * FROM tags
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
        let items = rows
            .into_iter()
            .map(TryInto::<Tag>::try_into)
            .collect::<ApplicationResult<Vec<Tag>>>()?;

        Ok(Page {
            next_page_token,
            items,
        })
    }
}

impl UpdateTag for Repository {
    #[tracing::instrument(skip_all)]
    async fn update_tag(&self, tag: &mut Tag) -> Result<(), ApplicationError> {
        let mut tx = self.begin_transaction().await?;

        let rec = sqlx::query!(
            r#"
UPDATE tags
SET title = $2, updated_at = DEFAULT
WHERE id = $1
RETURNING updated_at
            "#,
            tag.id(),
            tag.title().as_str()
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(from_sqlx_err)?;

        tag.set_updated_at(rec.updated_at)?;

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
    fn from_rows(rows: &[TagRow]) -> Option<Self> {
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

impl TryFrom<TagRow> for Tag {
    type Error = ApplicationError;

    fn try_from(value: TagRow) -> ApplicationResult<Self> {
        let TagRow {
            id,
            title,
            created_at,
            updated_at,
        } = value;

        let tag = Self::new(TagAttributes {
            id,
            title: TagTitle::new(title)?,
            created_at,
            updated_at,
        });

        Ok(tag)
    }
}

impl TryFrom<TagTitleRow> for TagTitle {
    type Error = ApplicationError;

    fn try_from(value: TagTitleRow) -> ApplicationResult<Self> {
        let TagTitleRow { title } = value;

        TagTitle::new(title)
    }
}

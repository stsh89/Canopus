pub mod remarks;
pub mod remarks_tags;
pub mod tags;

use base64::{
    alphabet,
    engine::{GeneralPurpose, general_purpose},
};
use canopus_definitions::{Error, Remark, Tag, Result};
use canopus_operations::{
    remarks::{
        DeleteRemark, GetRemark, InsertRemark, ListRemarks, NewRemark, RemarkUpdates,
        RemarksListing, RemarksListingParameters, UpdateRemark,
    },
    tags::{GetTag, ListTags, TagsListing, TagsListingParameters},
};
use chrono::{DateTime, Utc};
use remarks::RemarkRow;
use serde::{Deserialize, Serialize};
use sqlx::PgTransaction;
use tags::TagRow;
use uuid::Uuid;

const DEFAULT_PAGE_SIZE: i64 = 25;
const URL_SAFE_NO_PAD_ENGINE: GeneralPurpose =
    GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

pub struct Repository {
    pub pool: sqlx::PgPool,
}

#[derive(Serialize, Deserialize)]
pub struct PaginationToken {
    id: Uuid,
    created_at: DateTime<Utc>,
}

impl PaginationToken {
    fn from_string(token: String) -> eyre::Result<Self> {
        use base64::Engine;

        let json = URL_SAFE_NO_PAD_ENGINE.decode(token)?;

        let token = serde_json::from_slice(&json)?;

        Ok(token)
    }

    fn to_string(&self) -> eyre::Result<String> {
        use base64::Engine;

        let json = serde_json::to_string(&self)?;

        Ok(URL_SAFE_NO_PAD_ENGINE.encode(json))
    }
}

impl DeleteRemark for Repository {
    async fn delete_remark(&self, id: Uuid) -> Result<()> {
        delete_remark(self, id).await?;

        Ok(())
    }
}

impl GetRemark for Repository {
    async fn get_remark(&self, remark_id: Uuid) -> Result<Remark> {
        let remark = get_remark(self, remark_id).await.map_err(|err| match err {
            sqlx::Error::RowNotFound => Error::remark_not_found(remark_id),
            err => Error::unexpected(err),
        })?;

        Ok(remark)
    }
}

impl GetTag for Repository {
    async fn get_tag(&self, tag_id: Uuid) -> Result<Tag> {
        let tag = get_tag(self, tag_id).await.map_err(|err| match err {
            sqlx::Error::RowNotFound => Error::tag_not_found(tag_id),
            err => Error::unexpected(err),
        })?;

        Ok(tag)
    }
}

impl InsertRemark for Repository {
    async fn insert_remark(&self, new_remark: NewRemark) -> Result<Uuid> {
        let id = save_remark(self, new_remark).await.map_err(Into::<eyre::Error>::into)?;

        Ok(id)
    }
}

impl ListRemarks for Repository {
    async fn list_remarks(
        &self,
        parameters: RemarksListingParameters,
    ) -> Result<RemarksListing> {
        let listing = list_remarks(self, parameters).await?;

        Ok(listing)
    }
}

impl ListTags for Repository {
    async fn list_tags(
        &self,
        parameters: TagsListingParameters,
    ) -> Result<TagsListing> {
        let listing = list_tags(self, parameters).await?;

        Ok(listing)
    }
}

impl UpdateRemark for Repository {
    async fn update_remark(&self, parameters: RemarkUpdates) -> Result<()> {
        update_remark(self, parameters).await.map_err(Into::<eyre::Error>::into)?;

        Ok(())
    }
}

async fn assign_tags(tx: &mut PgTransaction<'_>, remark_id: Uuid, tags: Vec<String>) -> sqlx::Result<()> {
    for tag in tags {
        let tag_id = find_or_create_tag(tx, &tag).await?;

        if !remarks_tags::remark_tag_exists(tx, remark_id, tag_id).await? {
            remarks_tags::create(tx, remark_id, tag_id).await?;
        }
    }

    Ok(())
}

fn build_remarks_pagination_token(rows: &[RemarkRow]) -> Option<PaginationToken> {
    if rows.len() < DEFAULT_PAGE_SIZE as usize {
        return None;
    }

    rows.last().map(|row| PaginationToken {
        id: row.id,
        created_at: row.created_at,
    })
}

fn build_tags_pagination_token(rows: &[TagRow]) -> Option<PaginationToken> {
    if rows.len() < DEFAULT_PAGE_SIZE as usize {
        return None;
    }

    rows.last().map(|row| PaginationToken {
        id: row.id,
        created_at: row.created_at,
    })
}

async fn delete_remark(repository: &Repository, remark_id: Uuid) -> eyre::Result<()> {
    remarks::get_remark(&repository.pool, remark_id).await?;

    let mut tx = repository.pool.begin().await?;

    remarks::delete_remark(&mut tx, remark_id).await?;
    remarks_tags::delete_wasted_remarks_tags(&mut tx).await?;
    tags::delete_wasted_tags(&mut tx).await?;

    tx.commit().await?;

    Ok(())
}

async fn find_or_create_tag(tx: &mut PgTransaction<'_>, title: &str) -> sqlx::Result<uuid::Uuid> {
    if let Some(id) = tags::find(tx, title).await? {
        return Ok(id);
    };

    tags::create_tag(tx, title).await
}

async fn get_remark(repository: &Repository, id: Uuid) -> sqlx::Result<Remark> {
    let row = remarks::get_remark(&repository.pool, id).await?;
    let mut remark = Remark::from(row);

    let tags = remarks::list_tags(&repository.pool, id)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();

    remark.set_tags(tags);

    Ok(remark)
}

async fn get_tag(repository: &Repository, id: Uuid) -> sqlx::Result<Tag> {
    let row = tags::get_tag(&repository.pool, id).await?;

    Ok(row.into())
}

async fn list_remarks(
    repository: &Repository,
    parameters: RemarksListingParameters,
) -> eyre::Result<RemarksListing> {
    let RemarksListingParameters { pagination_token } = parameters;

    let pagination_token = pagination_token
        .map(PaginationToken::from_string)
        .transpose()?;

    let rows = remarks::list_remarks(&repository.pool, pagination_token).await?;

    let pagination_token = build_remarks_pagination_token(&rows)
        .map(|token| token.to_string())
        .transpose()?;

    Ok(RemarksListing {
        pagination_token,
        remarks: rows.into_iter().map(Into::into).collect(),
    })
}

async fn list_tags(
    repository: &Repository,
    parameters: TagsListingParameters,
) -> eyre::Result<TagsListing> {
    let TagsListingParameters { pagination_token } = parameters;

    let pagination_token = pagination_token
        .map(PaginationToken::from_string)
        .transpose()?;

    let rows = tags::list_tags(&repository.pool, pagination_token).await?;

    let pagination_token = build_tags_pagination_token(&rows)
        .map(|token| token.to_string())
        .transpose()?;

    Ok(TagsListing {
        pagination_token,
        tags: rows.into_iter().map(Into::into).collect(),
    })
}

async fn update_remark(repository: &Repository, parameters: RemarkUpdates) -> sqlx::Result<()> {
    let RemarkUpdates {
        id,
        essence,
        add_tags,
        remove_tags,
    } = parameters;

    let mut tx = repository.pool.begin().await?;

    assign_tags(&mut tx, id, add_tags).await?;

    if let Some(essence) = essence {
        remarks::update_remark(&mut tx, id, &essence).await?;
    }

    unset_tags(&mut tx, id, remove_tags).await?;
    tags::delete_wasted_tags(&mut tx).await?;

    tx.commit().await?;

    Ok(())
}

async fn save_remark(repository: &Repository, new_remark: NewRemark) -> sqlx::Result<Uuid> {
    let NewRemark { essence, tags } = new_remark;

    let mut tx = repository.pool.begin().await?;

    let remark_id = remarks::create_remark(&mut tx, &essence).await?;
    assign_tags(&mut tx, remark_id, tags).await?;

    tx.commit().await?;

    Ok(remark_id)
}

async fn unset_tags(tx: &mut PgTransaction<'_>, remark_id: Uuid, tags: Vec<String>) -> sqlx::Result<()> {
    for tag in tags {
        let Some(tag_id) = tags::find(tx, &tag).await? else {
            continue;
        };

        remarks_tags::delete(tx, remark_id, tag_id).await?;
    }

    Ok(())
}

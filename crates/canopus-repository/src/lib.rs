pub mod remarks;
pub mod remarks_tags;
pub mod tags;

use canopus_protocol::{
    remarks::{DeleteRemark, GetRemark, NewRemark, Remark, SaveRemark},
    tags::{GetTag, Tag},
};
use sqlx::PgTransaction;
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

type Result<T> = std::result::Result<T, Error>;

pub struct Repository {
    pub pool: sqlx::PgPool,
}

impl DeleteRemark for Repository {
    async fn delete_remark(&self, id: Uuid) -> canopus_protocol::Result<()> {
        delete_remark(self, id).await?;

        Ok(())
    }
}

impl GetRemark for Repository {
    async fn get_remark(&self, remark_id: Uuid) -> canopus_protocol::Result<Remark> {
        let remark = get_remark(self, remark_id).await?;

        Ok(remark)
    }
}

impl GetTag for Repository {
    async fn get_tag(&self, tag_id: Uuid) -> canopus_protocol::Result<Tag> {
        let tag = get_tag(self, tag_id).await?;

        Ok(tag)
    }
}

impl SaveRemark for Repository {
    async fn save_remark(&self, new_remark: NewRemark) -> canopus_protocol::Result<Uuid> {
        let id = save_remark(self, new_remark).await?;

        Ok(id)
    }
}

async fn delete_remark(repository: &Repository, remark_id: Uuid) -> Result<()> {
    remarks::get_remark(&repository.pool, remark_id).await?;

    let mut tx = repository.pool.begin().await?;

    remarks::delete_remark(&mut tx, remark_id).await?;
    remarks_tags::delete_wasted_remarks_tags(&mut tx).await?;
    tags::delete_wasted_tags(&mut tx).await?;

    tx.commit().await?;

    Ok(())
}

async fn get_remark(repository: &Repository, id: Uuid) -> Result<Remark> {
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

async fn get_tag(repository: &Repository, id: Uuid) -> Result<Tag> {
    let row = tags::get_tag(&repository.pool, id).await?;

    Ok(row.into())
}

async fn save_remark(repository: &Repository, new_remark: NewRemark) -> Result<Uuid> {
    let mut tx = repository.pool.begin().await?;
    let id = create_remark(&mut tx, new_remark).await?;
    tx.commit().await?;

    Ok(id)
}

impl From<Error> for canopus_protocol::Error {
    fn from(error: Error) -> Self {
        canopus_protocol::Error::Repository(error.into())
    }
}

async fn create_remark(tx: &mut PgTransaction<'_>, new_remark: NewRemark) -> Result<uuid::Uuid> {
    let NewRemark { essence, tags } = new_remark;

    let remark_id = remarks::create_remark(tx, &essence).await?;

    for tag in tags {
        let tag_id = find_or_create_tag(tx, &tag).await?;
        remarks_tags::create(tx, remark_id, tag_id).await?;
    }

    Ok(remark_id)
}

async fn find_or_create_tag(tx: &mut PgTransaction<'_>, title: &str) -> sqlx::Result<uuid::Uuid> {
    if let Some(id) = tags::find(tx, title).await? {
        return Ok(id);
    };

    tags::create_tag(tx, title).await
}

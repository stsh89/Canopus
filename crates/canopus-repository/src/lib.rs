use canopus_dataspine::storage::{remarks, remarks_tags, tags};
use canopus_protocol::{NewRemark, SaveRemark};
use sqlx::PgTransaction;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

type Result<T> = std::result::Result<T, Error>;

pub struct Repository {
    pub pool: sqlx::PgPool,
}

impl SaveRemark for Repository {
    async fn save_remark(&self, new_remark: NewRemark) -> canopus_protocol::Result<uuid::Uuid> {
        let id = save_remark(self, new_remark).await?;

        Ok(id)
    }
}

async fn save_remark(repository: &Repository, new_remark: NewRemark) -> Result<uuid::Uuid> {
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

    let remark_id = remarks::create(tx, &essence).await?;

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

    tags::create(tx, title).await
}

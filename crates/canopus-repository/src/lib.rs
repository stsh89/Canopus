mod remarks;
mod remarks_tags;
mod tags;

use base64::{
    alphabet,
    engine::{GeneralPurpose, general_purpose},
};
use canopus_definitions::{ApplicationError, ApplicationResult};
use sqlx::PgTransaction;
use tags::{TagRow, TagTitleRow};

const DEFAULT_PAGE_SIZE: i64 = 3;
const URL_SAFE_NO_PAD_ENGINE: GeneralPurpose =
    GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

pub struct Repository {
    pub pool: sqlx::PgPool,
}

impl Repository {
    async fn begin_transaction(&self) -> ApplicationResult<PgTransaction<'static>> {
        self.pool.begin().await.map_err(from_sqlx_err)
    }
}

async fn commit_transaction(tx: PgTransaction<'_>) -> ApplicationResult<()> {
    tx.commit().await.map_err(from_sqlx_err)
}

fn from_sqlx_err(err: sqlx::Error) -> ApplicationError {
    match err {
        sqlx::Error::RowNotFound => ApplicationError::NotFound,
        other => ApplicationError::Repository(other.to_string()),
    }
}

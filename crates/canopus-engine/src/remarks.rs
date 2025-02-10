pub use canopus_protocol::remarks::{NewRemark, Remark};

use crate::{Engine, Result};
use canopus_protocol::remarks;
use uuid::Uuid;

pub async fn create_remark(engine: &Engine, new_remark: NewRemark) -> Result<Uuid> {
    let repository = engine.repository();

    let id = remarks::create_remark(new_remark, repository).await?;

    Ok(id)
}

pub async fn get_remark(engine: &Engine, id: Uuid) -> Result<Remark> {
    let repository = engine.repository();

    let remark = remarks::get_remark(id, repository).await?;

    Ok(remark)
}

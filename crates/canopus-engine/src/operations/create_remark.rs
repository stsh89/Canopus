use crate::{Engine, Result};
use canopus_protocol::NewRemark;
use uuid::Uuid;

pub async fn create_remark(engine: &Engine, new_remark: NewRemark) -> Result<Uuid> {
    let repository = engine.repository();

    let id = canopus_protocol::create_remark(new_remark, repository).await?;

    Ok(id)
}

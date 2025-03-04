use canopus_definitions::Remark;
use canopus_operations::remarks;
pub use canopus_operations::remarks::{
    NewRemark, RemarkUpdates, RemarksListing, RemarksListingParameters,
};

use crate::{Engine, Result};
use uuid::Uuid;

pub async fn create_remark(engine: &Engine, new_remark: NewRemark) -> Result<Uuid> {
    let repository = engine.repository();

    let id = remarks::create_remark(new_remark, repository).await?;

    Ok(id)
}

pub async fn delete_remark(engine: &Engine, id: Uuid) -> Result<()> {
    let repository = engine.repository();

    remarks::delete_remark(id, repository).await?;

    Ok(())
}

pub async fn get_remark(engine: &Engine, id: Uuid) -> Result<Remark> {
    let repository = engine.repository();

    let remark = remarks::get_remark(id, repository).await?;

    Ok(remark)
}

pub async fn list_remarks(
    engine: &Engine,
    parameters: RemarksListingParameters,
) -> Result<RemarksListing> {
    let repository = engine.repository();

    let remarks = remarks::list_remarks(parameters, repository).await?;

    Ok(remarks)
}

pub async fn update_remark(engine: &Engine, parameters: RemarkUpdates) -> Result<()> {
    let repository = engine.repository();

    remarks::update_remark(parameters, repository).await?;

    Ok(())
}

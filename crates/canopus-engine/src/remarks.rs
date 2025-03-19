use canopus_definitions::{ApplicationResult, Remark};
use canopus_operations::remarks::{self, NewRemark, RemarkUpdates, RemarksListing, RemarksListingParameters};
use uuid::Uuid;
use crate::Engine;

pub async fn create_remark(engine: &Engine, new_remark: NewRemark) -> ApplicationResult<Uuid> {
    let Engine {
        repository,
    } = engine;

    let id = remarks::create_remark(new_remark, repository).await?;

    Ok(id)
}

pub async fn delete_remark(engine: &Engine, id: Uuid) -> ApplicationResult<()> {
    let Engine {
        repository,
    } = engine;

    remarks::delete_remark(id, repository).await?;

    Ok(())
}

pub async fn get_remark(engine: &Engine, id: Uuid) -> ApplicationResult<Remark> {
    let Engine {
        repository,
    } = engine;

    let remark = remarks::get_remark(id, repository).await?;

    Ok(remark)
}

pub async fn list_remarks(
    engine: &Engine,
    parameters: RemarksListingParameters,
) -> ApplicationResult<RemarksListing> {
    let Engine {
        repository,
    } = engine;

    let remarks = remarks::list_remarks(parameters, repository).await?;

    Ok(remarks)
}

pub async fn update_remark(engine: &Engine, parameters: RemarkUpdates) -> ApplicationResult<()> {
    let Engine {
        repository,
    } = engine;

    remarks::update_remark(parameters, repository).await?;

    Ok(())
}

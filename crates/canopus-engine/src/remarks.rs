use crate::Engine;
use canopus_definitions::{ApplicationResult, Page, Remark};
use canopus_operations::remarks::{
    self, NewRemarkAttributes, RemarkChanges, RemarksPageParameters,
};
use uuid::Uuid;

pub async fn create_remark(
    engine: &Engine,
    new_remark: NewRemarkAttributes,
) -> ApplicationResult<Remark> {
    let Engine { repository } = engine;

    remarks::create_remark(new_remark, repository).await
}

pub async fn delete_remark(engine: &Engine, id: Uuid) -> ApplicationResult<Remark> {
    let Engine { repository } = engine;

    remarks::delete_remark(id, repository).await
}

pub async fn get_remark(engine: &Engine, id: Uuid) -> ApplicationResult<Remark> {
    let Engine { repository } = engine;

    remarks::get_remark(id, repository).await
}

pub async fn list_remarks(
    engine: &Engine,
    parameters: RemarksPageParameters,
) -> ApplicationResult<Page<Remark>> {
    let Engine { repository } = engine;

    let page = remarks::list_remarks(parameters, repository).await?;

    Ok(page)
}

pub async fn update_remark(
    engine: &Engine,
    id: Uuid,
    changes: RemarkChanges,
) -> ApplicationResult<Remark> {
    let Engine { repository } = engine;

    remarks::update_remark(id, changes, repository).await
}

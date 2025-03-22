use crate::{
    Client, from_eyre,
    rest::{self, Path, Resource},
};
use canopus_definitions::{ApplicationResult, Page, Remark};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct NewRemark {
    pub essence: String,
    pub tags: Vec<String>,
}

#[derive(Default, Serialize)]
pub struct RemarkUpdates {
    pub essence: Option<String>,
    pub tags: Option<Vec<String>>,
}

pub async fn create(client: &Client, new_remark: NewRemark) -> ApplicationResult<Remark> {
    let Client { inner, base_url } = client;

    rest::create(
        inner,
        Resource {
            base_url,
            path: Path::Remarks,
        },
        new_remark,
    )
    .await
    .map_err(|err| from_eyre("failed to request remark creation", err))?
    .into()
}

pub async fn delete(client: &Client, id: Uuid) -> ApplicationResult<Remark> {
    let Client { inner, base_url } = client;

    rest::delete(
        inner,
        Resource {
            base_url,
            path: Path::Remark(id),
        },
    )
    .await
    .map_err(|err| from_eyre("failed to request remark deletion", err))?
    .into()
}

pub async fn index(client: &Client, page_token: Option<String>) -> ApplicationResult<Page<Remark>> {
    let Client { base_url, inner } = client;

    let query = page_token
        .as_deref()
        .map(|token| vec![("page_token", token)]);

    rest::get(
        inner,
        Resource {
            base_url,
            path: Path::Remarks,
        },
        query.as_deref(),
    )
    .await
    .map_err(|err| from_eyre("failed to request remarks page", err))?
    .into()
}

pub async fn show(client: &Client, id: Uuid) -> ApplicationResult<Remark> {
    let Client { base_url, inner } = client;

    rest::get(
        inner,
        Resource {
            base_url,
            path: Path::Remark(id),
        },
        None,
    )
    .await
    .map_err(|err| from_eyre("failed to request remark", err))?
    .into()
}

pub async fn update(
    client: &Client,
    id: Uuid,
    updates: RemarkUpdates,
) -> ApplicationResult<Remark> {
    let Client { base_url, inner } = client;

    rest::patch(
        inner,
        Resource {
            base_url,
            path: Path::Remark(id),
        },
        updates,
    )
    .await
    .map_err(|err| from_eyre("failed to update remark", err))?
    .into()
}

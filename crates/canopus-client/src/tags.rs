use crate::{
    Client, from_reqwest_err,
    rest::{self, Path, Resource},
};
use canopus_definitions::{ApplicationResult, Page, Tag};
use serde::Serialize;
use uuid::Uuid;

pub async fn index(client: &Client, page_token: Option<String>) -> ApplicationResult<Page<Tag>> {
    let Client { base_url, inner } = client;

    let query = page_token
        .as_deref()
        .map(|token| vec![("page_token", token)]);

    rest::get(
        inner,
        Resource {
            base_url,
            path: Path::Tags,
        },
        query.as_deref(),
    )
    .await
    .map_err(from_reqwest_err)?
    .into()
}

pub async fn show(client: &Client, id: Uuid) -> ApplicationResult<Tag> {
    let Client { base_url, inner } = client;

    rest::get(
        inner,
        Resource {
            base_url,
            path: Path::Tag(id),
        },
        None,
    )
    .await
    .map_err(from_reqwest_err)?
    .into()
}

pub async fn update(client: &Client, id: Uuid, title: String) -> ApplicationResult<Tag> {
    let Client { inner, base_url } = client;

    rest::patch(
        inner,
        Resource {
            base_url,
            path: Path::Tag(id),
        },
        TagChanges { title },
    )
    .await
    .map_err(from_reqwest_err)?
    .into()
}

#[derive(Serialize)]
struct TagChanges {
    title: String,
}

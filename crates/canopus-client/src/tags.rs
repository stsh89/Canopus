use crate::{Client, Path, Resource, from_eyre, get};
use canopus_definitions::{ApplicationResult, Page, Tag};
use uuid::Uuid;

pub async fn index(client: &Client, page_token: Option<String>) -> ApplicationResult<Page<Tag>> {
    let Client { base_url, inner } = client;

    let query = page_token
        .as_deref()
        .map(|token| vec![("page_token", token)]);

    get(
        inner,
        Resource {
            base_url,
            path: Path::Tags,
        },
        query.as_deref(),
    )
    .await
    .map_err(|err| from_eyre("failed to request tags page", err))?
    .into()
}

pub async fn show(client: &Client, id: Uuid) -> ApplicationResult<Tag> {
    let Client { base_url, inner } = client;

    get(
        inner,
        Resource {
            base_url,
            path: Path::Tag(id),
        },
        None,
    )
    .await
    .map_err(|err| from_eyre("failed to request tag", err))?
    .into()
}

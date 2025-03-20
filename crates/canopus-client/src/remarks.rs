use crate::{Client, Path, Resource, from_eyre, get};
use canopus_definitions::{ApplicationResult, Page, Remark};

pub async fn index(client: &Client, page_token: Option<String>) -> ApplicationResult<Page<Remark>> {
    let Client { base_url, inner } = client;

    let query = page_token
        .as_deref()
        .map(|token| vec![("page_token", token)]);

    get(
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

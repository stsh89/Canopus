use canopus_definitions::{Page, Tag};
use uuid::Uuid;

use crate::{ApiResponse, Client, Result};

pub async fn show(client: &Client, id: Uuid) -> Result<Tag> {
    let url = client.tags_url()?;

    client
        .inner
        .get(url)
        .send()
        .await?
        .json::<ApiResponse<Tag>>()
        .await?
        .into()
}

pub async fn index(client: &Client, page_token: Option<String>) -> Result<Page<Tag>> {
    let mut url = client
        .tags_url()?
        .join("/tags")
        .map_err(Into::<eyre::Error>::into)?;

    if let Some(page_token) = page_token {
        url.set_query(Some(&format!("page_token={}", page_token)));
    }

    client
        .inner
        .get(url)
        .send()
        .await?
        .json::<ApiResponse<Page<Tag>>>()
        .await?
        .into()
}

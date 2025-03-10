use canopus_definitions::{Page, Result, Tag};
use uuid::Uuid;

use crate::{ApiResponse, Client};

pub async fn show(client: &Client, id: Uuid) -> Result<Tag> {
    let url = client
        .tags_url()?
        .join(&format!("/tags/{}", id))
        .map_err(Into::<eyre::Error>::into)?;

    client
        .inner
        .get(url)
        .send()
        .await
        .map_err(Into::<eyre::Error>::into)?
        .json::<ApiResponse<Tag>>()
        .await
        .map_err(Into::<eyre::Error>::into)?
        .into()
}

pub async fn index(client: &Client, page_token: Option<String>) -> Result<Page<Tag>> {
    let mut url = client.tags_url()?;

    if let Some(page_token) = page_token {
        url.set_query(Some(&format!("page_token={}", page_token)));
    }

    client
        .inner
        .get(url)
        .send()
        .await
        .map_err(Into::<eyre::Error>::into)?
        .json::<ApiResponse<Page<Tag>>>()
        .await
        .map_err(Into::<eyre::Error>::into)?
        .into()
}

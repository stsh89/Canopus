use canopus_definitions::{ApplicationError, Page, Tag};
use reqwest::Url;
use serde::Deserialize;
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, Error>;

pub struct Client {
    inner: reqwest::Client,
    base_url: Url,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Connection error. Check if server is up and running")]
    Connection,

    #[error(transparent)]
    Application(#[from] ApplicationError),

    #[error("Internal client error")]
    Internal(#[from] eyre::Error),
}

#[derive(Deserialize)]
#[serde(untagged)]
enum GetTagResponse {
    Ok(Tag),
    Err(ApplicationError),
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ListTagsResponse {
    Ok(Page<Tag>),
    Err(ApplicationError),
}

impl Client {
    pub fn new() -> Result<Self> {
        let base_url: Url = "http://127.0.0.1:8000"
            .parse()
            .map_err(Into::<eyre::Error>::into)?;

        Ok(Self {
            base_url,
            inner: reqwest::Client::builder()
                .build()
                .map_err(eyre::Error::new)?,
        })
    }

    pub async fn show_tag(&self, id: Uuid) -> Result<Tag> {
        let url = self
            .base_url
            .join(&format!("/tags/{}", id))
            .map_err(Into::<eyre::Error>::into)?;

        let response = self
            .inner
            .get(url)
            .send()
            .await?
            .json::<GetTagResponse>()
            .await?;

        match response {
            GetTagResponse::Ok(tag) => Ok(tag.into()),
            GetTagResponse::Err(err) => Err(err.into()),
        }
    }

    pub async fn list_tags(&self, page_token: Option<String>) -> Result<Page<Tag>> {
        let mut url = self
            .base_url
            .join("/tags")
            .map_err(Into::<eyre::Error>::into)?;

        if let Some(page_token) = page_token {
            url.set_query(Some(&format!("page_token={}", page_token)));
        }

        let response = self
            .inner
            .get(url)
            .send()
            .await?
            .json::<ListTagsResponse>()
            .await?;

        match response {
            ListTagsResponse::Ok(page) => Ok(page.into()),
            ListTagsResponse::Err(err) => Err(err.into()),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        if value.is_connect() {
            Error::Connection
        } else {
            Self::Internal(value.into())
        }
    }
}

use canopus_definitions::{Page, Tag};
use canopus_wire::{ErrorMessage, PageMessage, TagMessage};
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

    #[error("Invalid argument error. {argument} {reason}")]
    InvalidArgument { argument: String, reason: String },

    #[error("Not found error. {resource} with ID {id} not found")]
    NotFound { resource: String, id: Uuid },

    #[error("Internal server error. Something went wrong on the server")]
    InternalServer,

    #[error("Internal client error")]
    Internal(#[from] eyre::Error),

    #[error("Unimplemented error. The operation is not implemented or not supported/enabled")]
    Unimplemented,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum GetTagResponse {
    Ok(TagMessage),
    Err(ErrorMessage),
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ListTagsResponse {
    Ok(PageMessage<TagMessage>),
    Err(ErrorMessage),
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

    pub async fn get_tag(&self, id: Uuid) -> Result<Tag> {
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

    pub async fn list_tags(&self, _page_token: Option<String>) -> Result<Page<Tag>> {
        let url = self
            .base_url
            .join("/tags")
            .map_err(Into::<eyre::Error>::into)?;

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

impl From<ErrorMessage> for Error {
    fn from(value: ErrorMessage) -> Self {
        match value {
            ErrorMessage::InvalidArgument { argument, reason } => {
                Error::InvalidArgument { argument, reason }
            }
            ErrorMessage::NotFound { resource, id } => Error::NotFound { resource, id },
            ErrorMessage::Internal => Error::InternalServer,
            ErrorMessage::Unimplemented => Error::Unimplemented,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        if value.is_connect() {
            Error::Connection
        } else {
            Self::InternalServer
        }
    }
}

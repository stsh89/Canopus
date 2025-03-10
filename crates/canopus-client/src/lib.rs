pub mod tags;

mod error;

pub use error::Error;

use canopus_definitions::ApplicationError;
use reqwest::Url;
use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum ApiResponse<T> {
    Ok(T),
    Err(ApplicationError),
}

pub struct Client {
    inner: reqwest::Client,
    base_url: Url,
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

    fn tags_url(&self) -> Result<Url> {
        let url = self
            .base_url
            .join("/tags")
            .map_err(Into::<eyre::Error>::into)?;

        Ok(url)
    }
}

impl<T> From<ApiResponse<T>> for Result<T> {
    fn from(value: ApiResponse<T>) -> Self {
        match value {
            ApiResponse::Ok(value) => Ok(value),
            ApiResponse::Err(err) => Err(Error::Application(err)),
        }
    }
}

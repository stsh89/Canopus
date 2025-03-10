pub mod tags;

use canopus_definitions::{ApplicationError, Result};
use reqwest::Url;
use serde::Deserialize;

#[derive(Deserialize)]
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
    pub fn new(base_url: &str) -> Result<Self> {
        let base_url: Url = base_url.parse().map_err(Into::<eyre::Error>::into)?;

        Ok(Self {
            base_url,
            inner: reqwest::Client::builder()
                .build()
                .map_err(Into::<eyre::Error>::into)?,
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
            ApiResponse::Err(err) => Err(err),
        }
    }
}

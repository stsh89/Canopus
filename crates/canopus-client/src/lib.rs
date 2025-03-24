pub mod remarks;
pub mod tags;

mod rest;

use canopus_definitions::{ApplicationError, ApplicationResult};
use reqwest::Url;
use std::env;

pub struct Client {
    inner: reqwest::Client,
    base_url: Url,
}

impl Client {
    pub fn from_env() -> ApplicationResult<Self> {
        let base_url_string = env::var("CANOPUS_BASE_URL")
            .map_err(|err| ApplicationError::internal("missing CANOPUS_BASE_URL env var", err))?;

        let base_url = Url::parse(&base_url_string).map_err(|err| {
            ApplicationError::internal("failed to parse CANOPUS_BASE_URL env var as URL", err)
        })?;

        let inner = reqwest::Client::builder()
            .build()
            .map_err(from_reqwest_err)?;

        Ok(Client { base_url, inner })
    }
}

fn from_reqwest_err(err: reqwest::Error) -> ApplicationError {
    ApplicationError::internal("client failed to communicate with the service", err)
}

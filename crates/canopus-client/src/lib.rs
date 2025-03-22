pub mod remarks;
pub mod tags;

mod rest;

use canopus_definitions::ApplicationError;
use eyre::WrapErr;
use reqwest::Url;
use std::env;

const SUBSYSTEM_NAME: &str = "Client";

pub struct Client {
    inner: reqwest::Client,
    base_url: Url,
}

impl Client {
    pub fn from_env() -> eyre::Result<Self> {
        let base_url_string =
            env::var("CANOPUS_BASE_URL").wrap_err_with(|| "Missing CANOPUS_BASE_URL env var")?;

        let base_url = Url::parse(&base_url_string)
            .wrap_err_with(|| "Failed to parse CANOPUS_BASE_URL env var as URL")?;

        let inner = reqwest::Client::builder()
            .build()
            .wrap_err_with(|| "Failed to build reqwest client")?;

        Ok(Client { base_url, inner })
    }
}

fn from_eyre(description: &str, report: eyre::Report) -> ApplicationError {
    ApplicationError::from_eyre(SUBSYSTEM_NAME, description, report)
}

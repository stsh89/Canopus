pub mod remarks;
pub mod tags;

use canopus_definitions::{ApplicationError, ApplicationResult};
use eyre::WrapErr;
use reqwest::Url;
use serde::Deserialize;
use std::env;
use uuid::Uuid;

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

#[derive(Deserialize)]
#[serde(untagged)]
enum ApiResponse<T> {
    Ok(T),
    Err(ApplicationError),
}

struct Resource<'a> {
    base_url: &'a Url,
    path: Path,
}

enum Path {
    Remarks,
    Tag(Uuid),
    Tags,
}

fn from_eyre(description: &str, report: eyre::Report) -> ApplicationError {
    ApplicationError::from_eyre(SUBSYSTEM_NAME, description, report)
}

async fn get<T>(
    client: &reqwest::Client,
    resource: Resource<'_>,
    query: Option<&[(&str, &str)]>,
) -> eyre::Result<ApiResponse<T>>
where
    T: for<'de> Deserialize<'de>,
{
    let url = Url::try_from(resource)?;

    let mut request = client.get(url);

    if let Some(query) = query {
        request = request.query(query);
    }

    let response = request.send().await?.json::<ApiResponse<T>>().await?;

    Ok(response)
}

impl TryFrom<Resource<'_>> for Url {
    type Error = eyre::Error;

    fn try_from(value: Resource) -> eyre::Result<Url> {
        let Resource { base_url, path } = value;

        let url = base_url.join(&path.to_string())?;

        Ok(url)
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Path::Remarks => f.write_str("/remarks"),
            Path::Tags => f.write_str("/tags"),
            Path::Tag(id) => write!(f, "{}/{}", Path::Tags, id),
        }
    }
}

impl<T> From<ApiResponse<T>> for ApplicationResult<T> {
    fn from(value: ApiResponse<T>) -> Self {
        match value {
            ApiResponse::Ok(value) => Ok(value),
            ApiResponse::Err(err) => Err(err),
        }
    }
}

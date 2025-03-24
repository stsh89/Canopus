use core::panic;

use canopus_definitions::{ApplicationError, ApplicationResult};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Ok(T),
    Err(ApplicationError),
}

pub struct Resource<'a> {
    pub base_url: &'a Url,
    pub path: Path,
}

pub enum Path {
    Remarks,
    Remark(Uuid),
    Tag(Uuid),
    Tags,
}

pub async fn create<T, D>(
    client: &reqwest::Client,
    url: impl Into<Url>,
    data: D,
) -> reqwest::Result<ApiResponse<T>>
where
    T: for<'de> Deserialize<'de>,
    D: Serialize,
{
    client
        .post(url.into())
        .json(&data)
        .send()
        .await?
        .json::<ApiResponse<T>>()
        .await
}

pub async fn delete<T>(
    client: &reqwest::Client,
    url: impl Into<Url>,
) -> reqwest::Result<ApiResponse<T>>
where
    T: for<'de> Deserialize<'de>,
{
    client
        .delete(url.into())
        .send()
        .await?
        .json::<ApiResponse<T>>()
        .await
}

pub async fn get<T>(
    client: &reqwest::Client,
    url: impl Into<Url>,
    query: Option<&[(&str, &str)]>,
) -> reqwest::Result<ApiResponse<T>>
where
    T: for<'de> Deserialize<'de>,
{
    let mut request = client.get(url.into());

    if let Some(query) = query {
        request = request.query(query);
    }

    request.send().await?.json::<ApiResponse<T>>().await
}

pub async fn patch<T, D>(
    client: &reqwest::Client,
    url: impl Into<Url>,
    data: D,
) -> reqwest::Result<ApiResponse<T>>
where
    T: for<'de> Deserialize<'de>,
    D: Serialize,
{
    client
        .patch(url.into())
        .json(&data)
        .send()
        .await?
        .json::<ApiResponse<T>>()
        .await
}

impl From<Resource<'_>> for Url {
    fn from(value: Resource) -> Url {
        let Resource { base_url, path } = value;

        base_url.join(&path.to_string()).unwrap_or_else(|_path| {
            panic!(
                "resource path should be a valid URL: '{}' is not a valid URL",
                path
            )
        })
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Path::Remarks => f.write_str("/remarks"),
            Path::Remark(id) => write!(f, "{}/{}", Path::Remarks, id),
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

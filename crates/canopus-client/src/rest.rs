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
    resource: Resource<'_>,
    data: D,
) -> eyre::Result<ApiResponse<T>>
where
    T: for<'de> Deserialize<'de>,
    D: Serialize,
{
    let url = Url::try_from(resource)?;

    let response = client
        .post(url)
        .json(&data)
        .send()
        .await?
        .json::<ApiResponse<T>>()
        .await?;

    Ok(response)
}

pub async fn delete<T>(
    client: &reqwest::Client,
    resource: Resource<'_>,
) -> eyre::Result<ApiResponse<T>>
where
    T: for<'de> Deserialize<'de>,
{
    let url = Url::try_from(resource)?;

    let response = client
        .delete(url)
        .send()
        .await?
        .json::<ApiResponse<T>>()
        .await?;

    Ok(response)
}

pub async fn get<T>(
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

pub async fn patch<T, D>(
    client: &reqwest::Client,
    resource: Resource<'_>,
    data: D,
) -> eyre::Result<ApiResponse<T>>
where
    T: for<'de> Deserialize<'de>,
    D: Serialize,
{
    let url = Url::try_from(resource)?;

    let response = client
        .patch(url)
        .json(&data)
        .send()
        .await?
        .json::<ApiResponse<T>>()
        .await?;

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

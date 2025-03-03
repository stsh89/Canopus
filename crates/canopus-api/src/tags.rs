use canopus_engine::{
    Engine,
    tags::{self, Tag, TagsListing, TagsListingParameters},
};
use chrono::{DateTime, Utc};
use rocket::{
    State,
    serde::{Serialize, json::Json},
};
use uuid::Uuid;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct TagPresenter {
    id: Uuid,
    title: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TagsPresenter {
    tags: Vec<TagPresenter>,
    page_token: Option<String>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorPresenter {
    error: String,
}

#[get("/?<page_token>")]
pub async fn index(
    engine: &State<Engine>,
    page_token: Option<String>,
) -> Result<Json<TagsPresenter>, Json<ErrorPresenter>> {
    let tags = tags::list_tags(
        engine,
        TagsListingParameters {
            pagination_token: page_token,
        },
    )
    .await
    .map_err(|err| Json(err.into()))?;

    Ok(Json(tags.into()))
}

impl From<Tag> for TagPresenter {
    fn from(value: Tag) -> Self {
        TagPresenter {
            id: value.id(),
            title: value.title().to_string(),
            created_at: value.created_at(),
            updated_at: value.updated_at(),
        }
    }
}

impl From<TagsListing> for TagsPresenter {
    fn from(value: TagsListing) -> Self {
        let TagsListing {
            tags,
            pagination_token,
        } = value;

        Self {
            tags: tags.into_iter().map(Into::into).collect(),
            page_token: pagination_token,
        }
    }
}

impl From<canopus_engine::Error> for ErrorPresenter {
    fn from(value: canopus_engine::Error) -> Self {
        let report = match value {
            canopus_engine::Error::EnvironmentVariable(report) => report,
            canopus_engine::Error::File(report) => report,
            canopus_engine::Error::Repository(report) => report,
        };

        Self {
            error: format!("{}", report),
        }
    }
}

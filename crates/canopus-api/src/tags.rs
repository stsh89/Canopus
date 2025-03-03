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

use crate::error::ApiError;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TagPresenter {
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

#[get("/?<page_token>")]
pub async fn index(
    engine: &State<Engine>,
    page_token: Option<String>,
) -> Result<Json<TagsPresenter>, ApiError> {
    let tags = tags::list_tags(
        engine,
        TagsListingParameters {
            pagination_token: page_token,
        },
    )
    .await?;

    Ok(Json(tags.into()))
}

#[get("/<id>")]
pub async fn get(engine: &State<Engine>, id: &str) -> Result<Json<TagPresenter>, ApiError> {
    let id = id
        .parse()
        .map_err(|_err| ApiError::bad_request("ID is not a valid UUID"))?;

    let tag = tags::get_tag(engine, id).await?;

    Ok(Json(tag.into()))
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

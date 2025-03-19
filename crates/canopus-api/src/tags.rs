use crate::error::Error;
use canopus_definitions::{Page, Tag};
use canopus_engine::{Engine, tags};
use canopus_operations::tags::ListTagsParameters;
use rocket::{State, serde::json::Json};

#[get("/?<page_token>")]
#[tracing::instrument(skip(engine), name = "Tags index", err(Debug))]
pub async fn index(
    engine: &State<Engine>,
    page_token: Option<String>,
) -> Result<Json<Page<Tag>>, Error> {
    let page = tags::list_tags(engine, ListTagsParameters { page_token }).await?;

    Ok(Json(page))
}

#[get("/<id>")]
#[tracing::instrument(skip(engine), name = "Tags show", err(Debug))]
pub async fn show(engine: &State<Engine>, id: &str) -> Result<Json<Tag>, Error> {
    let id = id.parse().map_err(|_err| Error::invalid_id())?;

    let tag = tags::get_tag(engine, id).await?;

    Ok(Json(tag))
}

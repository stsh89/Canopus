use crate::{error::Error, helpers};
use canopus_definitions::{Page, PageToken, Tag};
use canopus_engine::{Engine, tags};
use canopus_operations::tags::TagsPageParameters;
use rocket::{State, serde::json::Json};

#[get("/?<page_token>")]
#[tracing::instrument(skip(engine), name = "Tags index", err(Debug))]
pub async fn index(
    engine: &State<Engine>,
    page_token: Option<String>,
) -> Result<Json<Page<Tag>>, Error> {
    let page = tags::list_tags(
        engine,
        TagsPageParameters {
            page_token: page_token.map(PageToken::from),
        },
    )
    .await?;

    Ok(Json(page))
}

#[get("/<id>")]
#[tracing::instrument(skip(engine), name = "Tags show", err(Debug))]
pub async fn show(engine: &State<Engine>, id: &str) -> Result<Json<Tag>, Error> {
    let id = helpers::parse_id(id)?;

    let tag = tags::get_tag(engine, id).await?;

    Ok(Json(tag))
}

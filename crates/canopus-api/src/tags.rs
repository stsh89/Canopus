use crate::{Result, helpers};
use canopus_definitions::{Page, PageToken, Tag};
use canopus_engine::{Engine, tags};
use canopus_operations::tags::TagsPageParameters;
use rocket::{
    State,
    serde::{Deserialize, json::Json},
};

#[get("/?<page_token>")]
#[tracing::instrument(skip(engine), name = "Tags index", err(Debug))]
pub async fn index(engine: &State<Engine>, page_token: Option<String>) -> Result<Json<Page<Tag>>> {
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
pub async fn show(engine: &State<Engine>, id: &str) -> Result<Json<Tag>> {
    let id = helpers::parse_id(id)?;

    let tag = tags::get_tag(engine, id).await?;

    Ok(Json(tag))
}

#[patch("/<id>", data = "<form>")]
#[tracing::instrument(skip(engine), name = "Update tag", err(Debug))]
pub async fn update(
    engine: &State<Engine>,
    id: &str,
    form: Option<Json<UpdateTagForm>>,
) -> Result<Json<Tag>> {
    let id = helpers::parse_id(id)?;

    let title = form
        .map(|form| form.into_inner())
        .unwrap_or_default()
        .title
        .unwrap_or_default();

    let tag = tags::update_tag(engine, id, title).await?;

    Ok(Json(tag))
}

#[derive(Default, Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct UpdateTagForm {
    title: Option<String>,
}

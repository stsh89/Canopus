use crate::error::Error;
use canopus_engine::{
    Engine,
    tags::{self, TagsListingParameters},
};
use canopus_wire::{TagMessage, TagsMessage};
use rocket::{State, serde::json::Json};

#[get("/?<page_token>")]
pub async fn index(
    engine: &State<Engine>,
    page_token: Option<String>,
) -> Result<Json<TagsMessage>, Error> {
    let tags = tags::list_tags(
        engine,
        TagsListingParameters {
            pagination_token: page_token,
        },
    )
    .await?;

    Ok(Json(TagsMessage {
        page: tags.tags.into_iter().map(Into::into).collect(),
        next_page_token: tags.pagination_token,
    }))
}

#[get("/<id>")]
pub async fn find(engine: &State<Engine>, id: &str) -> Result<Json<TagMessage>, Error> {
    let id = id
        .parse()
        .map_err(|_err| Error::bad_request("ID is not a valid UUID"))?;

    let tag = tags::get_tag(engine, id).await?;

    Ok(Json(tag.into()))
}

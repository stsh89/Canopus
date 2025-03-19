use crate::error::Error;
use canopus_definitions::{Page, Remark};
use canopus_engine::{Engine, remarks};
use canopus_operations::remarks::RemarksListingParameters;
use rocket::{State, serde::json::Json};

#[get("/?<page_token>")]
#[tracing::instrument(skip(engine), name = "Remarks index", err(Debug))]
pub async fn index(
    engine: &State<Engine>,
    page_token: Option<String>,
) -> Result<Json<Page<Remark>>, Error> {
    let page = remarks::list_remarks(engine, RemarksListingParameters { page_token }).await?;

    Ok(Json(page))
}

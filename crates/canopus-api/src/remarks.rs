use crate::Result;
use canopus_definitions::{Page, Remark};
use canopus_engine::{Engine, remarks};
use canopus_operations::remarks::{NewRemark, RemarksListingParameters};
use rocket::{
    State,
    serde::{Deserialize, json::Json},
};

#[post("/", data = "<form>")]
#[tracing::instrument(skip(engine), name = "Create remakr", err(Debug))]
pub async fn create(engine: &State<Engine>, form: Json<RemarkForm>) -> Result<Json<Remark>> {
    let RemarkForm { essence, tags } = form.into_inner();

    let new_remark = NewRemark { essence, tags };
    let id = remarks::create_remark(engine, new_remark).await?;
    let remark = remarks::get_remark(engine, id).await?;

    Ok(Json(remark))
}

#[get("/?<page_token>")]
#[tracing::instrument(skip(engine), name = "Remarks index", err(Debug))]
pub async fn index(
    engine: &State<Engine>,
    page_token: Option<String>,
) -> Result<Json<Page<Remark>>> {
    let page = remarks::list_remarks(engine, RemarksListingParameters { page_token }).await?;

    Ok(Json(page))
}

// #[derive(Deserialize)]
#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct RemarkForm {
    essence: String,
    tags: Vec<String>,
}

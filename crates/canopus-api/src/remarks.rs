use crate::{Result, helpers};
use canopus_definitions::{Page, PageToken, Remark};
use canopus_engine::{Engine, remarks};
use canopus_operations::remarks::{NewRemarkAttributes, RemarkChanges, RemarksPageParameters};
use rocket::{
    State,
    serde::{Deserialize, json::Json},
};

#[post("/", data = "<form>")]
#[tracing::instrument(skip(engine), name = "Create remark", err(Debug))]
pub async fn create(
    engine: &State<Engine>,
    form: Option<Json<NewRemarkForm>>,
) -> Result<Json<Remark>> {
    let new_remark_attributes = form
        .map(|form| form.into_inner().into())
        .unwrap_or_else(NewRemarkAttributes::empty);

    let remark = remarks::create_remark(engine, new_remark_attributes).await?;

    Ok(Json(remark))
}

#[delete("/<id>")]
#[tracing::instrument(skip(engine), name = "Delete remark", err(Debug))]
pub async fn delete(engine: &State<Engine>, id: &str) -> Result<Json<Remark>> {
    let id = helpers::parse_id(id)?;

    let remark = remarks::delete_remark(engine, id).await?;

    Ok(Json(remark))
}

#[get("/?<page_token>")]
#[tracing::instrument(skip(engine), name = "Remarks index", err(Debug))]
pub async fn index(
    engine: &State<Engine>,
    page_token: Option<String>,
) -> Result<Json<Page<Remark>>> {
    let page = remarks::list_remarks(
        engine,
        RemarksPageParameters {
            page_token: page_token.map(PageToken::from),
        },
    )
    .await?;

    Ok(Json(page))
}

#[get("/<id>")]
#[tracing::instrument(skip(engine), name = "Show remark", err(Debug))]
pub async fn show(engine: &State<Engine>, id: &str) -> Result<Json<Remark>> {
    let id = helpers::parse_id(id)?;

    let remark = remarks::get_remark(engine, id).await?;

    Ok(Json(remark))
}

#[patch("/<id>", data = "<form>")]
#[tracing::instrument(skip(engine), name = "Update remark", err(Debug))]
pub async fn update(
    engine: &State<Engine>,
    id: &str,
    form: Option<Json<UpdateRemarkForm>>,
) -> Result<Json<Remark>> {
    let id = helpers::parse_id(id)?;

    let changes = form
        .map(|form| form.into_inner().into())
        .unwrap_or_else(RemarkChanges::empty);

    let remark = remarks::update_remark(engine, id, changes).await?;

    Ok(Json(remark))
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct NewRemarkForm {
    essence: Option<String>,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct UpdateRemarkForm {
    essence: Option<String>,
    tags: Option<Vec<String>>,
}

impl From<NewRemarkForm> for NewRemarkAttributes {
    fn from(value: NewRemarkForm) -> Self {
        let NewRemarkForm { essence, tags } = value;

        NewRemarkAttributes {
            essence: essence.unwrap_or_default(),
            tags: tags.unwrap_or_default(),
        }
    }
}

impl From<UpdateRemarkForm> for RemarkChanges {
    fn from(value: UpdateRemarkForm) -> Self {
        let UpdateRemarkForm { essence, tags } = value;

        RemarkChanges { essence, tags }
    }
}

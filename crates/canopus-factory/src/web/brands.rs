use super::{ServerState, WebResult, helpers};
use crate::brands::{self, Brand, CreateBrandParameters};
use chrono::{DateTime, Utc};
use rocket::{
    State, delete, get, post,
    serde::{Deserialize, Serialize, json::Json},
};
use uuid::Uuid;

#[post("/", data = "<form>")]
pub async fn create(
    state: &State<ServerState>,
    form: Option<Json<NewBrandForm>>,
) -> WebResult<Json<BrandPresenter>> {
    let form = form.map(|form| form.into_inner()).unwrap_or_default();
    let brand = brands::create_brand(&state.repo, form.into()).await?;

    Ok(Json(brand.into()))
}

#[get("/")]
pub async fn index(state: &State<ServerState>) -> WebResult<Json<Vec<BrandPresenter>>> {
    let brands = brands::list_brands(&state.repo).await?;

    Ok(Json(brands.into_iter().map(Into::into).collect()))
}

#[delete("/<id>")]
pub async fn delete(state: &State<ServerState>, id: &str) -> WebResult<Json<BrandPresenter>> {
    let id = helpers::parse_id(id)?;
    let brand = brands::delete_brand(&state.repo, id).await?;

    Ok(Json(brand.into()))
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct BrandPresenter {
    id: Uuid,
    name: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewBrandForm {
    name: Option<String>,
}

impl From<NewBrandForm> for CreateBrandParameters {
    fn from(form: NewBrandForm) -> Self {
        let NewBrandForm { name } = form;

        CreateBrandParameters {
            name: name.unwrap_or_default(),
        }
    }
}

impl From<Brand> for BrandPresenter {
    fn from(value: Brand) -> Self {
        BrandPresenter {
            id: value.id(),
            name: value.name().to_string(),
            created_at: value.created_at(),
            updated_at: value.updated_at(),
        }
    }
}

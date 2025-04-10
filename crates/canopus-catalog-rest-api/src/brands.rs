use crate::{Result, ServerState, helpers};
use canopus_catalog::{Brand, CreateBrand, CreateBrandParameters, DeleteBrand, ListBrands, Record};
use chrono::{DateTime, Utc};
use rocket::{
    State,
    serde::{Deserialize, Serialize, json::Json},
};
use uuid::Uuid;

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

#[post("/", data = "<form>")]
pub async fn create(
    engine: &State<ServerState>,
    form: Option<Json<NewBrandForm>>,
) -> Result<Json<BrandPresenter>> {
    let NewBrandForm { name } = form.map(|form| form.into_inner()).unwrap_or_default();

    let brand = CreateBrand { repo: &engine.repo }
        .execute(CreateBrandParameters {
            name: name.unwrap_or_default(),
        })
        .await?;

    Ok(Json(brand.into()))
}

#[delete("/<id>")]
pub async fn delete(engine: &State<ServerState>, id: &str) -> Result<Json<BrandPresenter>> {
    let id = helpers::parse_id(id)?;

    let brand = DeleteBrand { repo: &engine.repo }.execute(id).await?;

    Ok(Json(brand.into()))
}

#[get("/")]
pub async fn index(state: &State<ServerState>) -> Result<Json<Vec<BrandPresenter>>> {
    let brands = ListBrands { repo: &state.repo }.execute().await?;

    Ok(Json(brands.into_iter().map(Into::into).collect()))
}

impl From<Record<Brand>> for BrandPresenter {
    fn from(record: Record<Brand>) -> Self {
        let Record {
            id,
            data: brand,
            created_at,
            updated_at,
        } = record;

        Self {
            id,
            name: brand.name().to_string(),
            created_at,
            updated_at,
        }
    }
}

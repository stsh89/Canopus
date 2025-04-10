mod brands;
mod helpers;

use canopus_catalog::Error as CatalogError;
use canopus_catalog_repo::Repository as CatalogRepository;
use eyre::Context;
use rocket::serde::{Deserialize, Serialize, json::Json};

#[macro_use]
extern crate rocket;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Config {
    repo: RepoConfig,
    server: ServerConfig,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct RepoConfig {
    url: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct ServerConfig {
    port: u16,
}

struct ServerState {
    repo: CatalogRepository,
}

#[derive(Responder)]
pub enum Error {
    #[response(status = 400, content_type = "json")]
    BadRequest(Json<ErrorPresenter>),

    #[response(status = 500, content_type = "json")]
    Internal(Json<ErrorPresenter>),

    #[response(status = 404, content_type = "json")]
    NotFound(Json<ErrorPresenter>),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorPresenter {
    error: ErrorKind,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde", rename_all = "snake_case")]
enum ErrorKind {
    InvalidArgument(String),
    RecordAlreadyExists,
    RecordNotFound,
    Repo(String),
    Validation(String),
}

type Result<T> = std::result::Result<T, Error>;

#[rocket::main]
async fn main() -> eyre::Result<()> {
    let config_string =
        std::fs::read_to_string("config.toml").wrap_err("failed to read config file")?;

    let Config { repo, server } =
        toml::from_str(&config_string).wrap_err("failed to parse config file")?;

    let repo = CatalogRepository::new(&repo.url).await?;
    let figment = rocket::Config::figment().merge(("port", server.port));

    let _rocket = rocket::custom(figment)
        .mount("/brands", routes![brands::index])
        .mount("/brands", routes![brands::create])
        .mount("/brands", routes![brands::delete])
        .manage(ServerState { repo })
        .launch()
        .await?;

    Ok(())
}

impl From<CatalogError> for ErrorKind {
    fn from(err: CatalogError) -> Self {
        match err {
            CatalogError::Validation(description) => ErrorKind::Validation(description),
            CatalogError::RecordNotFound => ErrorKind::RecordNotFound,
            CatalogError::RecordAlreadyExists => ErrorKind::RecordAlreadyExists,
            CatalogError::Repo(description) => ErrorKind::Repo(description),
            CatalogError::InvalidArgument(description) => ErrorKind::InvalidArgument(description),
        }
    }
}

impl From<CatalogError> for Error {
    fn from(err: CatalogError) -> Self {
        match err {
            CatalogError::Validation(_) => {
                Self::BadRequest(Json(ErrorPresenter { error: err.into() }))
            }
            CatalogError::RecordNotFound => {
                Self::NotFound(Json(ErrorPresenter { error: err.into() }))
            }
            CatalogError::RecordAlreadyExists => {
                Self::BadRequest(Json(ErrorPresenter { error: err.into() }))
            }
            CatalogError::Repo(_) => Self::Internal(Json(ErrorPresenter { error: err.into() })),
            CatalogError::InvalidArgument(_) => {
                Self::BadRequest(Json(ErrorPresenter { error: err.into() }))
            }
        }
    }
}

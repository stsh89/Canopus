pub mod brands;
mod helpers;

use crate::{AppError, repo::Repo};
use rocket::{
    Responder, routes,
    serde::{Serialize, json::Json},
};

pub type WebResult<T> = std::result::Result<T, WebError>;

pub async fn run(repo: Repo, config: ServerConfig) -> eyre::Result<()> {
    let ServerConfig { port } = config;

    let figment = rocket::Config::figment().merge(("port", port));

    let _rocket = rocket::custom(figment)
        .mount("/brands", routes![brands::create])
        .mount("/brands", routes![brands::index])
        .mount("/brands", routes![brands::delete])
        .manage(ServerState { repo })
        .launch()
        .await?;

    Ok(())
}

pub struct ServerState {
    repo: Repo,
}

pub struct ServerConfig {
    pub port: u16,
}

#[derive(Responder)]
pub enum WebError {
    #[response(status = 400, content_type = "json")]
    BadRequest(Json<ErrorPresenter>),

    #[response(status = 500, content_type = "json")]
    Internal(Json<ErrorPresenter>),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorPresenter {
    error: ErrorKind,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde", rename_all = "snake_case")]
enum ErrorKind {
    FailedPrecodition(String),
    InvalidArgument(String),
    ResourceAlreadyExists(String),
    ResourceNotFound(String),
    Repo(String),
    Validation(String),
}

impl From<AppError> for ErrorKind {
    fn from(err: AppError) -> Self {
        match err {
            AppError::FailedPrecodition(description) => ErrorKind::FailedPrecodition(description),
            AppError::InvalidArgument(description) => ErrorKind::InvalidArgument(description),
            AppError::ResourceAlreadyExists(description) => {
                ErrorKind::ResourceAlreadyExists(description)
            }
            AppError::ResourceNotFound(description) => ErrorKind::ResourceNotFound(description),
            AppError::Repo(description) => ErrorKind::Repo(description.to_string()),
            AppError::Validation(description) => ErrorKind::Validation(description),
        }
    }
}

impl From<AppError> for WebError {
    fn from(err: AppError) -> Self {
        match err {
            AppError::FailedPrecodition(_) => {
                Self::BadRequest(Json(ErrorPresenter { error: err.into() }))
            }
            AppError::InvalidArgument(_) => {
                Self::BadRequest(Json(ErrorPresenter { error: err.into() }))
            }
            AppError::ResourceAlreadyExists(_) => {
                Self::BadRequest(Json(ErrorPresenter { error: err.into() }))
            }
            AppError::ResourceNotFound(_) => {
                Self::BadRequest(Json(ErrorPresenter { error: err.into() }))
            }
            AppError::Repo(_) => Self::Internal(Json(ErrorPresenter { error: err.into() })),
            AppError::Validation(_) => Self::BadRequest(Json(ErrorPresenter { error: err.into() })),
        }
    }
}

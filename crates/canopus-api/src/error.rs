use rocket::serde::{Serialize, json::Json};

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
    pub error: String,
}

impl From<canopus_definitions::Error> for Error {
    fn from(value: canopus_definitions::Error) -> Self {
        let content = Json(ErrorPresenter {
            error: value.to_string(),
        });

        match value {
            canopus_definitions::Error::NotFound { .. } => Error::NotFound(content),
            canopus_definitions::Error::Unexpected(_) => Error::Internal(content),
        }
    }
}

impl Error {
    pub fn bad_request(message: &str) -> Self {
        Self::BadRequest(Json(ErrorPresenter {
            error: message.to_string(),
        }))
    }
}

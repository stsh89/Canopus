use rocket::serde::{Serialize, json::Json};

#[derive(Responder)]
pub enum ApiError {
    #[response(status = 400, content_type = "json")]
    BadRequest(Json<ErrorPresenter>),

    #[response(status = 500, content_type = "json")]
    Internal(()),
    // #[response(status = 404, content_type = "json")]
    // NotFound(()),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorPresenter {
    pub error: String,
}

impl From<canopus_engine::EngineError> for ApiError {
    fn from(_value: canopus_engine::EngineError) -> Self {
        Self::Internal(())
    }
}

impl ApiError {
    pub fn bad_request(message: &str) -> Self {
        Self::BadRequest(Json(ErrorPresenter {
            error: message.to_string(),
        }))
    }
}

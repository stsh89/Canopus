use canopus_definitions::ApplicationError;
use rocket::serde::json::Json;

#[derive(Responder)]
pub enum Error {
    #[response(status = 400, content_type = "json")]
    InvalidArgument(Json<ApplicationError>),

    #[response(status = 500, content_type = "json")]
    Internal(Json<ApplicationError>),

    #[response(status = 404, content_type = "json")]
    NotFound(Json<ApplicationError>),
}

impl Error {
    fn bad_request(err: ApplicationError) -> Self {
        Self::InvalidArgument(Json(err))
    }

    fn not_found(err: ApplicationError) -> Self {
        Self::NotFound(Json(err))
    }

    pub fn invalid_id() -> Self {
        Self::InvalidArgument(Json(ApplicationError::InvalidArgument {
            argument: "ID".to_string(),
            reason: "is not a valid UUID".to_string(),
        }))
    }

    pub fn internal(err: ApplicationError) -> Self {
        Self::Internal(Json(err))
    }
}

impl From<ApplicationError> for Error {
    fn from(value: ApplicationError) -> Self {
        match value {
            ApplicationError::InvalidArgument { .. } => Error::bad_request(value),
            ApplicationError::NotFound { .. } => Error::not_found(value),
            ApplicationError::Internal(_) => Error::internal(value),
        }
    }
}

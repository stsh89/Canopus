use canopus_definitions::ApplicationError;
use rocket::serde::json::Json;

#[derive(Debug, Responder)]
pub enum Error {
    #[response(status = 400, content_type = "json")]
    InvalidArgument(Json<ApplicationError>),

    #[response(status = 500, content_type = "json")]
    Internal(Json<ApplicationError>),

    #[response(status = 404, content_type = "json")]
    NotFound(Json<ApplicationError>),

    #[response(status = 501, content_type = "json")]
    Unimplemented(Json<ApplicationError>),
}

impl Error {
    fn bad_request(err: ApplicationError) -> Self {
        Self::InvalidArgument(Json(err))
    }

    pub fn invalid_id() -> Self {
        Self::InvalidArgument(Json(ApplicationError::invalid_argument(
            "ID is not a valid UUID",
        )))
    }

    pub fn internal(err: ApplicationError) -> Self {
        Self::Internal(Json(err))
    }
}

impl From<ApplicationError> for Error {
    fn from(value: ApplicationError) -> Self {
        match value {
            ApplicationError::InvalidArgument { .. } => Error::bad_request(value),
            ApplicationError::NotFound => Error::NotFound(Json(value)),
            ApplicationError::Internal { .. } => Error::internal(value),
            ApplicationError::Unimplemented => Error::Unimplemented(Json(value)),
            ApplicationError::Repository(_) => Error::internal(value),
        }
    }
}

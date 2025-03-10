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

    #[response(status = 404, content_type = "json")]
    Unknown(Json<ApplicationError>),
}

impl Error {
    fn bad_request(message: ApplicationError) -> Self {
        Self::InvalidArgument(Json(message))
    }

    fn not_found(message: ApplicationError) -> Self {
        Self::NotFound(Json(message))
    }

    pub fn invalid_id() -> Self {
        Self::InvalidArgument(Json(ApplicationError::InvalidArgument {
            argument: "ID".to_string(),
            reason: "is not a valid UUID".to_string(),
        }))
    }

    fn internal() -> Self {
        Self::Internal(Json(ApplicationError::Internal))
    }

    pub fn unknown() -> Self {
        Self::Unknown(Json(ApplicationError::Unknown))
    }
}

impl From<ApplicationError> for Error {
    fn from(value: ApplicationError) -> Self {
        match value {
            ApplicationError::InvalidArgument { .. } => Error::bad_request(value),
            ApplicationError::NotFound { .. } => Error::not_found(value),
            ApplicationError::Internal => Error::internal(),
            ApplicationError::Unknown => Error::unknown(),
        }
    }
}

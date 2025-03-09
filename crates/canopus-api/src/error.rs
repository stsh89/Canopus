use canopus_definitions::ApplicationError;
use canopus_engine::Error as EngineError;
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

impl From<EngineError> for Error {
    fn from(value: EngineError) -> Self {
        match value {
            EngineError::ApplicationError(application_error) => match application_error {
                ApplicationError::InvalidArgument { argument, reason } => {
                    Error::bad_request(ApplicationError::InvalidArgument { argument, reason })
                }
                ApplicationError::NotFound { resource, id } => {
                    Error::not_found(ApplicationError::NotFound { resource, id })
                }
                ApplicationError::Internal => Error::internal(),
                ApplicationError::Unknown => Error::unknown(),
            },
            EngineError::Internal(_report) => Error::internal(),
        }
    }
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

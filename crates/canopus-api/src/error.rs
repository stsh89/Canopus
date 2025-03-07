use canopus_definitions::ApplicationError;
use canopus_engine::Error as EngineError;
use canopus_wire::ErrorMessage;
use rocket::serde::json::Json;

#[derive(Responder)]
pub enum Error {
    #[response(status = 400, content_type = "json")]
    BadRequest(Json<ErrorMessage>),

    #[response(status = 500, content_type = "json")]
    Internal(Json<ErrorMessage>),

    #[response(status = 404, content_type = "json")]
    NotFound(Json<ErrorMessage>),

    #[response(status = 404, content_type = "json")]
    Unimplemented(Json<ErrorMessage>),
}

impl From<EngineError> for Error {
    fn from(value: EngineError) -> Self {
        match value {
            EngineError::ApplicationError(application_error) => match application_error {
                ApplicationError::InvalidArgument { argument, reason } => {
                    Error::bad_request(ErrorMessage::InvalidArgument { argument, reason })
                }
                ApplicationError::NotFound { resource, id } => {
                    Error::not_found(ErrorMessage::NotFound { resource, id })
                }
                ApplicationError::Internal(_report) => Error::internal(),
            },
            EngineError::Internal(_report) => Error::internal(),
        }
    }
}

impl Error {
    fn bad_request(message: ErrorMessage) -> Self {
        Self::BadRequest(Json(message))
    }

    fn not_found(message: ErrorMessage) -> Self {
        Self::NotFound(Json(message))
    }

    pub fn invalid_id() -> Self {
        Self::BadRequest(Json(ErrorMessage::InvalidArgument {
            argument: "ID".to_string(),
            reason: "is not a valid UUID".to_string(),
        }))
    }

    pub fn unimplemented() -> Self {
        Self::Unimplemented(Json(ErrorMessage::Unimplemented))
    }

    fn internal() -> Self {
        Self::Internal(Json(ErrorMessage::Internal))
    }
}

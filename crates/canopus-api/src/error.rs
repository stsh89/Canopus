use canopus_definitions::Error as CanopusError;
use canopus_wire::{ErrorMessage, InvalidArgumentErrorMessage};
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

impl From<CanopusError> for Error {
    fn from(value: CanopusError) -> Self {
        let message = ErrorMessage::from(value);

        match message {
            ErrorMessage::InvalidArgument(_) => Self::BadRequest(Json(message)),
            ErrorMessage::NotFound(_) => Self::NotFound(Json(message)),
            ErrorMessage::Internal(_) => Self::Internal(Json(message)),
            ErrorMessage::Unimplemented(_) => Self::NotFound(Json(message)),
        }
    }
}

impl Error {
    pub fn invalid_id() -> Self {
        Self::BadRequest(Json(ErrorMessage::InvalidArgument(
            InvalidArgumentErrorMessage {
                argument: "ID".to_string(),
                reason: "is not a valid UUID".to_string(),
            },
        )))
    }

    pub fn unimplemented() -> Self {
        Self::Unimplemented(Json(ErrorMessage::unimplemented()))
    }
}

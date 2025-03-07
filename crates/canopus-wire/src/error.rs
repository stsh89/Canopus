use canopus_definitions::ApplicationError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorMessage {
    #[serde(rename = "invalid_argument_error")]
    InvalidArgument { argument: String, reason: String },

    #[serde(rename = "not_found_error")]
    NotFound { resource: String, id: Uuid },

    #[serde(rename = "internal_error")]
    Internal,

    #[serde(rename = "unimplemented_error")]
    Unimplemented,
}

impl From<ApplicationError> for ErrorMessage {
    fn from(value: ApplicationError) -> Self {
        match value {
            ApplicationError::InvalidArgument { argument, reason } => {
                Self::InvalidArgument { argument, reason }
            }
            ApplicationError::NotFound { resource, id } => Self::NotFound { resource, id },
            ApplicationError::Internal(_report) => Self::Internal,
        }
    }
}

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(thiserror::Error, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplicationError {
    #[error("Invalid argument error. {argument} {reason}")]
    InvalidArgument { argument: String, reason: String },

    #[error("Not found error. {resource} with ID {id} not found")]
    NotFound { resource: String, id: Uuid },

    #[error("Internal application error. Something went wrong")]
    Internal,

    #[error("Unknown application error.")]
    Unknown,
}

impl ApplicationError {
    pub fn remark_not_found(id: Uuid) -> Self {
        ApplicationError::NotFound {
            resource: "Remark".to_string(),
            id,
        }
    }

    pub fn tag_not_found(id: Uuid) -> Self {
        ApplicationError::NotFound {
            resource: "Tag".to_string(),
            id,
        }
    }
}

impl From<eyre::Error> for ApplicationError {
    fn from(value: eyre::Error) -> Self {
        // TODO: Replace with tracing
        eprintln!("Error: {}", value);

        ApplicationError::Internal
    }
}

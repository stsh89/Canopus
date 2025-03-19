use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(thiserror::Error, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplicationError {
    #[error("{0}")]
    #[serde(rename = "invalid_argument_error")]
    InvalidArgument(String),

    #[error("{resource} with ID {id} not found")]
    #[serde(rename = "not_found_error")]
    NotFound { resource: String, id: Uuid },

    #[error("{subsystem} {description}. Details: {details}")]
    #[serde(rename = "internal_error")]
    Internal {
        subsystem: String,
        description: String,
        details: String,
    },

    #[error("The operation is not implemented or is not supported/enabled in this service.")]
    #[serde(rename = "unimplemented_error")]
    Unimplemented,
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

    pub fn invalid_argument(description: &str) -> Self {
        ApplicationError::InvalidArgument(description.to_string())
    }

    pub fn from_eyre(subsystem: &str, descripton: &str, report: eyre::Report) -> Self {
        ApplicationError::Internal {
            subsystem: subsystem.to_string(),
            description: descripton.to_string(),
            details: format!("{:?}", report),
        }
    }
}

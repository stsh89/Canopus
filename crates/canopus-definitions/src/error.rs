use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplicationError {
    #[error("{subsystem} {description}. Details: {details}")]
    Internal {
        subsystem: String,
        description: String,
        details: String,
    },

    #[error("{0}")]
    InvalidArgument(String),

    #[error("The requested entity was not found")]
    NotFound,

    #[error("{0}")]
    Repository(String),

    #[error("The operation is not implemented or is not supported/enabled in this service.")]
    Unimplemented,
}

impl ApplicationError {
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

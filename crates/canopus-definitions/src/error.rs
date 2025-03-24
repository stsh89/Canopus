use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplicationError {
    #[error("{description}")]
    Internal {
        description: String,
        details: Option<String>,
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
    pub fn from_eyre(description: &str, err: eyre::Report) -> Self {
        ApplicationError::Internal {
            description: description.to_string(),
            details: Some(format!("{:}", err)),
        }
    }

    pub fn internal(descripton: &str, err: impl std::error::Error) -> Self {
        ApplicationError::Internal {
            description: descripton.to_string(),
            details: Some(format!("{:?}", err)),
        }
    }

    pub fn invalid_argument(description: &str) -> Self {
        ApplicationError::InvalidArgument(description.to_string())
    }

    pub fn msg(description: &str) -> Self {
        ApplicationError::Internal {
            description: description.to_string(),
            details: None,
        }
    }

    pub fn repository(description: &str) -> Self {
        ApplicationError::Repository(description.to_string())
    }
}

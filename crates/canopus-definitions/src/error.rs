use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum ApplicationError {
    #[error("Invalid argument error. {argument} {reason}")]
    InvalidArgument { argument: String, reason: String },

    #[error("Not found error. {resource} with ID {id} not found")]
    NotFound { resource: String, id: Uuid },

    #[error("Internal application error")]
    Internal(#[from] eyre::Error),
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

    pub fn unexpected(err: impl Into<eyre::Error>) -> Self {
        ApplicationError::Internal(err.into())
    }
}

use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid argument: {argument} {reason}")]
    InvalidArgument {
        argument: &'static str,
        reason: &'static str,
    },

    #[error("{resource} with ID {id} not found")]
    NotFound { resource: &'static str, id: Uuid },

    #[error("Unexpected failure. Please try again later")]
    Internal(#[from] eyre::Error),

    #[error("The operation is not implemented or not supported/enabled")]
    Unimplemented,
}

impl Error {
    pub fn remark_not_found(id: Uuid) -> Self {
        Error::NotFound {
            resource: "Remark",
            id,
        }
    }

    pub fn tag_not_found(id: Uuid) -> Self {
        Error::NotFound {
            resource: "Tag",
            id,
        }
    }

    pub fn unexpected(err: impl Into<eyre::Error>) -> Self {
        Error::Internal(err.into())
    }
}

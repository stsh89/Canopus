use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{resource} with ID {id} not found")]
    NotFound {
        resource: &'static str,
        id: Uuid,
    },

    #[error("Unexpected failure. Please try again later")]
    Unexpected(#[from] eyre::Error),
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
        Error::Unexpected(err.into())
    }
}

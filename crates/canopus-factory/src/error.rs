#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed precondition: {0}")]
    FailedPrecodition(String),

    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    #[error("not found: {0}")]
    ResourceNotFound(String),

    #[error("repository failure: {0}")]
    Repo(#[from] sqlx::Error),

    #[error("resource already exists: {0}")]
    ResourceAlreadyExists(String),

    #[error("validation error: {0}")]
    Validation(String),
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("validation error: {0}")]
    Validation(String),

    #[error("requested record not found")]
    RecordNotFound,

    #[error("record already exists")]
    RecordAlreadyExists,

    #[error("repository error: {0}")]
    Repo(String),

    #[error("invalid argument: {0}")]
    InvalidArgument(String),
}

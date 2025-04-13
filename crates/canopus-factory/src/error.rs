#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("validation error: {0}")]
    Validation(String),

    #[error("repository failure: {0}")]
    Repo(#[from] sqlx::Error),

    #[error("resource already exists: {0}")]
    ResourceAlreadyExists(String),
}

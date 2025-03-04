#[derive(thiserror::Error, Debug)]
pub enum CanopusError {
    #[error("Repository failure: {0}")]
    Repository(#[source] eyre::Error),
}

pub mod remarks;
pub mod tags;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Repository failure: {0}")]
    Repository(#[source] eyre::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

mod create_remark;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Repository failure: {0}")]
    Repository(#[source] eyre::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub use create_remark::*;

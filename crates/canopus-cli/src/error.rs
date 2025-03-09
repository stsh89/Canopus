use std::io;

use canopus_client::Error as ClientError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Client(#[from] ClientError),

    #[error(r#"Internal CLI error."#)]
    Internal(#[from] eyre::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Internal(eyre::Error::new(err))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Internal(eyre::Error::new(err))
    }
}

use canopus_definitions::ApplicationError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Connection error. Check if server is up and running")]
    Connection,

    #[error(transparent)]
    Application(#[from] ApplicationError),

    #[error("Internal client error")]
    Internal(#[from] eyre::Error),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        if value.is_connect() {
            Error::Connection
        } else {
            Self::Internal(value.into())
        }
    }
}

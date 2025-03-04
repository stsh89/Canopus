use canopus_definitions::{Error, Tag};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct TagMessage {
    pub id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize)]
pub struct TagsMessage {
    pub page: Vec<TagMessage>,
    pub next_page_token: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorMessage {
    InvalidArgument(InvalidArgumentErrorMessage),
    NotFound(NotFoundErrorMessage),
    Internal(InternalErrorMessage),
    Unimplemented(UnimplmenetedErrorMessage),
}

#[derive(Deserialize, Serialize)]
pub struct InvalidArgumentErrorMessage {
    pub argument: String,
    pub reason: String,
}

#[derive(Deserialize, Serialize)]
pub struct NotFoundErrorMessage {
    pub resource: String,
    pub id: Uuid,
}

#[derive(Deserialize, Serialize)]
pub struct InternalErrorMessage {
    pub message: String,
}

#[derive(Deserialize, Serialize)]
pub struct UnimplmenetedErrorMessage {
    pub message: String,
}

impl From<Tag> for TagMessage {
    fn from(value: Tag) -> Self {
        TagMessage {
            id: value.id(),
            title: value.title().to_string(),
            created_at: value.created_at(),
            updated_at: value.updated_at(),
        }
    }
}

impl From<Error> for ErrorMessage {
    fn from(value: Error) -> Self {
        match value {
            Error::InvalidArgument { argument, reason } => {
                ErrorMessage::InvalidArgument(InvalidArgumentErrorMessage {
                    argument: argument.to_string(),
                    reason: reason.to_string(),
                })
            }
            Error::NotFound { resource, id } => ErrorMessage::NotFound(NotFoundErrorMessage {
                resource: resource.to_string(),
                id,
            }),
            Error::Internal(_report) => ErrorMessage::internal(),
            Error::Unimplemented => ErrorMessage::unimplemented(),
        }
    }
}

impl ErrorMessage {
    pub fn unimplemented() -> Self {
        ErrorMessage::Unimplemented(UnimplmenetedErrorMessage {
            message: "The operation is not implemented or not supported/enabled".to_string(),
        })
    }

    pub fn internal() -> Self {
        ErrorMessage::Internal(InternalErrorMessage {
            message: "Unexpected failure. Please try again later".to_string(),
        })
    }
}

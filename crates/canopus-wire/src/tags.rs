use canopus_definitions::{Tag, TagAttributes};
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

impl From<TagMessage> for Tag {
    fn from(value: TagMessage) -> Self {
        let TagMessage {
            id,
            title,
            created_at,
            updated_at,
        } = value;

        Self::new(TagAttributes {
            id,
            title,
            created_at,
            updated_at,
        })
    }
}

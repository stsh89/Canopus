use canopus_definitions::Tag;
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

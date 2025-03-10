use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Tag {
    id: Uuid,
    title: TagTitle,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub struct TagAttributes {
    pub id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TagTitle(String);

impl Tag {
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn new(attributes: TagAttributes) -> Self {
        let TagAttributes {
            id,
            title,
            created_at,
            updated_at,
        } = attributes;

        Self {
            id,
            title: TagTitle(title),
            created_at,
            updated_at,
        }
    }

    pub fn title(&self) -> &TagTitle {
        &self.title
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl std::ops::Deref for TagTitle {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string_pretty(&self).map_err(|_| std::fmt::Error)?;

        f.write_str(&json)
    }
}

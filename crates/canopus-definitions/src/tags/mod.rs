mod tag_title;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use tag_title::TagTitle;

use crate::{ApplicationError, ApplicationResult};

#[derive(Debug, Deserialize, Serialize)]
pub struct Tag {
    id: Uuid,
    title: TagTitle,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub struct TagAttributes {
    pub id: Uuid,
    pub title: TagTitle,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

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
            title,
            created_at,
            updated_at,
        }
    }

    pub fn set_title(&mut self, title: TagTitle) {
        self.title = title;
    }

    pub fn set_updated_at(&mut self, updated_at: DateTime<Utc>) -> ApplicationResult<()> {
        if self.updated_at > updated_at {
            return Err(ApplicationError::invalid_argument(
                "updated_at must be greater than current updated_at",
            ));
        }

        self.updated_at = updated_at;

        Ok(())
    }

    pub fn title(&self) -> &TagTitle {
        &self.title
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string_pretty(&self).map_err(|_err| std::fmt::Error)?;

        f.write_str(&json)
    }
}

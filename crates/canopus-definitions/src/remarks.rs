use crate::Tag;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Remark {
    id: Uuid,
    essence: RemarkEssence,
    tags: Vec<Tag>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub struct RemarkAttributes {
    pub id: Uuid,
    pub essence: String,
    pub tags: Vec<Tag>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RemarkEssence(String);

impl Remark {
    pub fn add_tag(&mut self, tag: Tag) {
        self.tags.push(tag);
    }

    pub fn set_tags(&mut self, tags: Vec<Tag>) {
        self.tags = tags;
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn essence(&self) -> &RemarkEssence {
        &self.essence
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn new(attributes: RemarkAttributes) -> Self {
        let RemarkAttributes {
            id,
            essence,
            tags,
            created_at,
            updated_at,
        } = attributes;

        Remark {
            id,
            essence: RemarkEssence(essence),
            tags,
            created_at,
            updated_at,
        }
    }

    pub fn tags(&self) -> &[Tag] {
        &self.tags
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl std::ops::Deref for RemarkEssence {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

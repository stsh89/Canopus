mod remark_essence;

pub use remark_essence::RemarkEssence;

use crate::{ApplicationError, ApplicationResult, TagTitle};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Remark {
    id: Uuid,
    essence: RemarkEssence,
    tags: BTreeSet<TagTitle>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub struct RemarkAttributes {
    pub id: Uuid,
    pub essence: RemarkEssence,
    pub tags: Vec<TagTitle>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Remark {
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
            essence,
            tags: BTreeSet::from_iter(tags),
            created_at,
            updated_at,
        }
    }

    pub fn set_essence(&mut self, essence: RemarkEssence) {
        self.essence = essence;
    }

    pub fn set_tags(&mut self, tags: Vec<TagTitle>) {
        self.tags = BTreeSet::from_iter(tags);
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

    pub fn tags(&self) -> Vec<&TagTitle> {
        self.tags.iter().collect()
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl std::fmt::Display for Remark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string_pretty(&self).map_err(|_| std::fmt::Error)?;

        f.write_str(&json)
    }
}

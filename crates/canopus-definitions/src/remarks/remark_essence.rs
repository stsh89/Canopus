use serde::{Deserialize, Serialize};
use crate::{ApplicationError, ApplicationResult};

#[derive(Debug, Deserialize, Serialize)]
pub struct RemarkEssence(String);

impl RemarkEssence {
    pub fn new(essence: String) -> ApplicationResult<Self> {
        if essence.is_empty() {
            return Err(ApplicationError::invalid_argument(
                "remark essence can't be blank",
            ));
        }

        Ok(Self(essence.trim().to_string()))
    }
}

impl std::ops::Deref for RemarkEssence {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

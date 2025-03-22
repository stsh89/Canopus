use crate::{ApplicationError, ApplicationResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, PartialOrd, Ord)]
pub struct TagTitle(String);

impl TagTitle {
    pub fn new(value: String) -> ApplicationResult<Self> {
        if value.is_empty() {
            return Err(ApplicationError::invalid_argument(
                "tag title can't be blank",
            ));
        }

        Ok(Self(value.trim().to_lowercase()))
    }
}

impl std::ops::Deref for TagTitle {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

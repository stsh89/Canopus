use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PageToken(String);

impl std::ops::Deref for PageToken {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for PageToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PageToken {{ {} }}", self.0)
    }
}

impl From<String> for PageToken {
    fn from(value: String) -> Self {
        Self(value)
    }
}

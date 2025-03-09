use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Page<T>
where
    T: Serialize,
{
    pub items: Vec<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<PageToken>,
}

#[derive(Deserialize, Serialize)]
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

impl<T> std::fmt::Display for Page<T>
where
    T: Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?;

        f.write_str(&json)
    }
}

impl From<String> for PageToken {
    fn from(value: String) -> Self {
        Self(value)
    }
}

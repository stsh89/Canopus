mod page_token;

pub use page_token::PageToken;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Page<T>
where
    T: Serialize,
{
    pub items: Vec<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<PageToken>,
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

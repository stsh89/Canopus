mod error;
mod tags;

use canopus_definitions::Page;
pub use error::*;
pub use tags::*;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PageMessage<T>
where
    T: Serialize,
{
    pub items: Vec<T>,
    pub next_page_token: Option<String>,
}

impl<T, E> From<PageMessage<T>> for Page<E>
where
    T: Serialize,
    E: From<T>,
{
    fn from(value: PageMessage<T>) -> Self {
        let PageMessage {
            items,
            next_page_token,
        } = value;

        Page {
            items: items.into_iter().map(Into::into).collect(),
            next_page_token,
        }
    }
}

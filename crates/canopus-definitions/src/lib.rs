mod error;
mod remarks;
mod tags;

pub use error::ApplicationError;
pub use remarks::{Remark, RemarkAttributes, RemarkEssence};
pub use tags::{Tag, TagAttributes, TagTitle};

pub struct Page<T> {
    pub items: Vec<T>,
    pub next_page_token: Option<String>,
}

mod error;
mod page;
mod remarks;
mod tags;

pub use error::ApplicationError;
pub use page::{Page, PageToken};
pub use remarks::{Remark, RemarkAttributes, RemarkEssence};
pub use tags::{Tag, TagAttributes, TagTitle};

pub type ApplicationResult<T> = std::result::Result<T, ApplicationError>;

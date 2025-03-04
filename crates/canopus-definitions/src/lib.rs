mod tags;
mod error;
mod remarks;

pub use tags::{Tag, TagAttributes, TagTitle};
pub use remarks::{Remark, RemarkAttributes, RemarkEssence};
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;

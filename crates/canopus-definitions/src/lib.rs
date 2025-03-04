mod error;
mod remarks;
mod tags;

pub use error::Error;
pub use remarks::{Remark, RemarkAttributes, RemarkEssence};
pub use tags::{Tag, TagAttributes, TagTitle};

pub type Result<T> = std::result::Result<T, Error>;

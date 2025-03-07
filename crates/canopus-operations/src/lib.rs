use canopus_definitions::ApplicationError;

pub mod remarks;
pub mod tags;

type Result<T> = std::result::Result<T, ApplicationError>;

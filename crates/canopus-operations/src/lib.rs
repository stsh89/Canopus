use canopus_definitions::CanopusError;

pub mod remarks;
pub mod tags;

type Result<T> = std::result::Result<T, CanopusError>;

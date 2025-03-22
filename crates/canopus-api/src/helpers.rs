use crate::{Error, Result};
use uuid::Uuid;

pub fn parse_id(id: &str) -> Result<Uuid> {
    id.parse().map_err(|_err| Error::invalid_id())
}

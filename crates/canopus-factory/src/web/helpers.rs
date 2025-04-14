use crate::AppError;

use super::WebResult;
use uuid::Uuid;

pub fn parse_id(id: &str) -> WebResult<Uuid> {
    id.parse()
        .map_err(|_err| AppError::InvalidArgument("ID is not a valid UUID".to_string()).into())
}

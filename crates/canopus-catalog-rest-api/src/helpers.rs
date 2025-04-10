use crate::{Error, ErrorKind, ErrorPresenter, Result};
use rocket::serde::json::Json;
use uuid::Uuid;

pub fn parse_id(id: &str) -> Result<Uuid> {
    id.parse().map_err(|_err| {
        Error::BadRequest(Json(ErrorPresenter {
            error: ErrorKind::InvalidArgument("ID is not a valid UUID".to_string()),
        }))
    })
}

mod error;
mod helpers;
mod remarks;
mod tags;
mod tracing;

use canopus_definitions::ApplicationError;
use canopus_engine::Engine;
use error::Error;
use rocket::{Request, serde::json::Json};

type Result<T> = std::result::Result<T, Error>;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> eyre::Result<()> {
    let _guard = tracing::init_subscriber();
    let engine = Engine::start().await?;

    let _rocket = rocket::build()
        .mount("/tags", routes![tags::index])
        .mount("/tags", routes![tags::show])
        .mount("/remarks", routes![remarks::create])
        .mount("/remarks", routes![remarks::delete])
        .mount("/remarks", routes![remarks::index])
        .mount("/remarks", routes![remarks::show])
        .mount("/remarks", routes![remarks::update])
        .register("/", catchers![not_found, internal_error])
        .manage(engine)
        .launch()
        .await?;

    Ok(())
}

#[catch(404)]
fn not_found(_req: &Request) -> Error {
    Error::Unimplemented(Json(ApplicationError::Unimplemented))
}

#[catch(500)]
fn internal_error(req: &Request) -> Error {
    Error::Unimplemented(Json(ApplicationError::from_eyre(
        "API",
        "faced unexpected error",
        eyre::Report::msg(req.to_string()),
    )))
}

mod error;
mod tags;
mod tracing;

use canopus_definitions::ApplicationError;
use canopus_engine::Engine;
use error::Error;
use rocket::{Request, serde::json::Json};

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> eyre::Result<()> {
    let _guard = tracing::init_subscriber();
    let engine = Engine::start().await?;

    let _rocket = rocket::build()
        .mount("/tags", routes![tags::index])
        .mount("/tags", routes![tags::show])
        .register("/", catchers![not_found])
        .manage(engine)
        .launch()
        .await?;

    Ok(())
}

#[catch(404)]
fn not_found(_req: &Request) -> Error {
    Error::Unimplemented(Json(ApplicationError::Unimplemented))
}

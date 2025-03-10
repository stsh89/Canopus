mod error;
mod tags;

use canopus_engine::Engine;
use error::Error;
use rocket::Request;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> anyhow::Result<()> {
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
    Error::internal(eyre::Error::msg("Unmatched route").into())
}

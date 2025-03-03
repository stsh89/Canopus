mod error;
mod tags;

use canopus_engine::Engine;
use error::ErrorPresenter;
use rocket::{Request, serde::json::Json};

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let engine = Engine::start().await?;

    let _rocket = rocket::build()
        .mount("/tags", routes![tags::index])
        .mount("/tags", routes![tags::get])
        .register("/", catchers![not_found])
        .manage(engine)
        .launch()
        .await?;

    Ok(())
}

#[catch(404)]
fn not_found(_req: &Request) -> Json<ErrorPresenter> {
    Json(ErrorPresenter {
        error: "Resource not found. Available resources: remarks, tags".to_string(),
    })
}

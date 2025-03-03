mod error;
mod tags;

use canopus_engine::Engine;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let engine = Engine::start().await?;

    let _rocket = rocket::build()
        .mount("/tags", routes![tags::index])
        .mount("/tags", routes![tags::get])
        .manage(engine)
        .launch()
        .await?;

    Ok(())
}

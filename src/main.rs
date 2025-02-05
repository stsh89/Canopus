mod storage;

use sqlx::postgres::PgPool;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenvy::dotenv().expect("Failed to load .env file");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;

    let id = storage::remarks::create(&pool, "Hello").await?;
    storage::remarks::update(&pool, id, "World").await?;
    storage::remarks::get(&pool, id).await?;
    storage::remarks::delete(&pool, id).await?;

    Ok(())
}

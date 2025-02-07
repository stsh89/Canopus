use crate::support;
use canopus_dataspine::storage::tags;
use sqlx::PgPool;

#[sqlx::test(fixtures("tags"))]
async fn it_gets_tag(pool: PgPool) -> sqlx::Result<()> {
    let id = support::get_first_tag(&pool).await?;
    let result = tags::get(&pool, id).await;

    assert!(result.is_ok());

    Ok(())
}

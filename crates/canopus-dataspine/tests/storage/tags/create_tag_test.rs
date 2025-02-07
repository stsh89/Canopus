use crate::support;
use canopus_dataspine::storage::tags;
use sqlx::PgPool;

#[sqlx::test]
async fn it_creates_tag(pool: PgPool) -> sqlx::Result<()> {
    let count_was = support::count_tags(&pool).await?;
    tags::create(&pool, "Philosophy").await?;

    let count = support::count_tags(&pool).await?;
    assert_eq!(count, count_was + 1);

    Ok(())
}

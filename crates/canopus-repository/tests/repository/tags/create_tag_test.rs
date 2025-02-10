use crate::support;
use canopus_repository::tags;
use sqlx::PgPool;

#[sqlx::test]
async fn it_creates_tag(pool: PgPool) -> sqlx::Result<()> {
    let count_was = support::count_tags(&pool).await?;

    let mut tx = pool.begin().await?;
    tags::create_tag(&mut tx, "Philosophy").await?;
    tx.commit().await?;

    let count = support::count_tags(&pool).await?;
    assert_eq!(count, count_was + 1);

    Ok(())
}

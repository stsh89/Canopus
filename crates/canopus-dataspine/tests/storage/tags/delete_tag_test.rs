use crate::support;
use canopus_dataspine::storage::tags;
use sqlx::PgPool;

#[sqlx::test(fixtures("tags"))]
fn it_deletes_tag(pool: PgPool) -> sqlx::Result<()> {
    let id = support::get_first_tag(&pool).await?;

    let count = tags::delete(&pool, id).await?;
    assert_eq!(count, 1);

    let tag_exists = support::tag_exists(&pool, id).await?;
    assert!(!tag_exists);

    Ok(())
}

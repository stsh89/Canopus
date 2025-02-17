use crate::support;
use canopus_repository::remarks_tags;
use sqlx::PgPool;

#[sqlx::test(fixtures("remarks_tags"))]
fn it_deletes_remark(pool: PgPool) -> sqlx::Result<()> {
    let remark_id = support::get_first_remark_with_tag(&pool).await?;
    let tag_id = support::get_first_tag(&pool).await?;

    let mut tx = pool.begin().await?;
    let count = remarks_tags::delete(&mut tx, remark_id, tag_id).await?;
    tx.commit().await?;
    assert_eq!(count, 1);

    let remark_tag_exists = support::remark_tag_exists(&pool, remark_id, tag_id).await?;
    assert!(!remark_tag_exists);

    Ok(())
}

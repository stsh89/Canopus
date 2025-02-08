use crate::support;
use canopus_dataspine::storage::remarks_tags;
use sqlx::PgPool;

#[sqlx::test(fixtures("remarks_tags"))]
async fn it_creates_remark_tag(pool: PgPool) -> sqlx::Result<()> {
    let count_was = support::count_remarks_tags(&pool).await?;
    let remark_id = support::get_first_remark_without_tag(&pool).await?;
    let tag_id = support::get_first_tag(&pool).await?;

    let mut tx = pool.begin().await?;
    remarks_tags::create(&mut tx, remark_id, tag_id).await?;
    tx.commit().await?;

    let count = support::count_remarks_tags(&pool).await?;
    assert_eq!(count, count_was + 1);

    Ok(())
}

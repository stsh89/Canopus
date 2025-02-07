use crate::support;
use canopus_dataspine::storage::remarks;
use sqlx::PgPool;

#[sqlx::test]
async fn it_creates_remark(pool: PgPool) -> sqlx::Result<()> {
    let count_was = support::count_remarks(&pool).await?;
    remarks::create(&pool, "The only source of knowledge is experience").await?;

    let count = support::count_remarks(&pool).await?;
    assert_eq!(count, count_was + 1);

    Ok(())
}

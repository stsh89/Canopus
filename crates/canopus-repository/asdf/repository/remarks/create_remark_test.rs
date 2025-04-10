use crate::support;
use canopus_repository::remarks;
use sqlx::PgPool;

#[sqlx::test]
async fn it_creates_remark(pool: PgPool) -> sqlx::Result<()> {
    let count_was = support::count_remarks(&pool).await?;

    let mut tx = pool.begin().await?;
    remarks::create_remark(&mut tx, "The only source of knowledge is experience").await?;
    tx.commit().await?;

    let count = support::count_remarks(&pool).await?;
    assert_eq!(count, count_was + 1);

    Ok(())
}

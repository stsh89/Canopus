use crate::support;
use canopus_repository::remarks;
use sqlx::PgPool;

#[sqlx::test(fixtures("remarks"))]
fn it_deletes_remark(pool: PgPool) -> sqlx::Result<()> {
    let id = support::get_first_remark(&pool).await?;

    let mut tx = pool.begin().await?;
    let count = remarks::delete_remark(&mut tx, id).await?;
    tx.commit().await?;

    assert_eq!(count, 1);

    let remark_exists = support::remark_exists(&pool, id).await?;
    assert!(!remark_exists);

    Ok(())
}

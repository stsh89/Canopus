use crate::support;
use canopus_dataspine::storage::remarks;
use sqlx::PgPool;

#[sqlx::test(fixtures("remarks"))]
fn it_deletes_remark(pool: PgPool) -> sqlx::Result<()> {
    let id = support::get_first_remark(&pool).await?;

    let count = remarks::delete(&pool, id).await?;
    assert_eq!(count, 1);

    let remark_exists = support::remark_exists(&pool, id).await?;
    assert!(!remark_exists);

    Ok(())
}

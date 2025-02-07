use canopus_dataspine::storage::remarks;
use sqlx::PgPool;
use crate::support;

#[sqlx::test(fixtures("remarks"))]
async fn it_gets_remark(pool: PgPool) -> sqlx::Result<()> {
    let id = support::get_first_remark(&pool).await?;
    let result = remarks::get(&pool, id).await;

    assert!(result.is_ok());

    Ok(())
}

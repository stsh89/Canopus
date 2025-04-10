use crate::support;
use canopus_repository::remarks;
use sqlx::PgPool;

#[sqlx::test(fixtures("remarks"))]
async fn it_gets_remark(pool: PgPool) -> sqlx::Result<()> {
    let id = support::get_first_remark(&pool).await?;
    let result = remarks::get_remark(&pool, id).await;

    assert!(result.is_ok());

    Ok(())
}

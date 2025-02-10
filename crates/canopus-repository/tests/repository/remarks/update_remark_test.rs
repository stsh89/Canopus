use crate::support;
use canopus_repository::remarks;
use sqlx::PgPool;

#[sqlx::test(fixtures("remarks"))]
async fn it_updates_remark(pool: PgPool) -> sqlx::Result<()> {
    let id = support::get_first_remark(&pool).await?;
    let essence_was = support::get_remark_essence(&pool, id).await?;
    let new_essence = "No great thing is created suddenly";
    assert!(essence_was != new_essence);

    remarks::update(&pool, id, new_essence).await?;

    let essence = support::get_remark_essence(&pool, id).await?;
    assert!(essence == new_essence);

    Ok(())
}

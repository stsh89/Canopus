use crate::support;
use canopus_dataspine::storage::tags;
use sqlx::PgPool;

#[sqlx::test(fixtures("tags"))]
async fn it_updates_tag(pool: PgPool) -> sqlx::Result<()> {
    let id = support::get_first_tag(&pool).await?;
    let title_was = support::get_tag_title(&pool, id).await?;
    let new_title = "cybernetics";
    assert!(title_was != new_title);

    tags::update(&pool, id, new_title).await?;

    let essence = support::get_tag_title(&pool, id).await?;
    assert!(essence == new_title);

    Ok(())
}

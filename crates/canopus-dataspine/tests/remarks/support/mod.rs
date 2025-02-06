use sqlx::PgPool;
use uuid::Uuid;

pub async fn count_remarks(pool: &PgPool) -> sqlx::Result<i64> {
    let count = sqlx::query_scalar!(r#"SELECT COUNT(*) as "count!" FROM remarks"#)
        .fetch_one(pool)
        .await?;

    Ok(count)
}

pub async fn get_first_remark(pool: &PgPool) -> sqlx::Result<Uuid> {
    let id = sqlx::query_scalar!(r#"SELECT id FROM remarks ORDER BY created_at DESC LIMIT 1"#)
        .fetch_one(pool)
        .await?;

    Ok(id)
}

pub async fn get_remark_essence(pool: &PgPool, id: Uuid) -> sqlx::Result<String> {
    let essence = sqlx::query_scalar!(r#"SELECT essence FROM remarks WHERE id = $1"#, id)
        .fetch_one(pool)
        .await?;

    Ok(essence)
}

pub async fn remark_exists(pool: &PgPool, id: Uuid) -> sqlx::Result<bool> {
    let exists = sqlx::query_scalar!(
        r#"SELECT EXISTS(SELECT 1 FROM remarks WHERE id = $1) as "exists!""#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(exists)
}

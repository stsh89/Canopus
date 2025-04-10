use sqlx::PgPool;
use uuid::Uuid;

pub async fn count_remarks(pool: &PgPool) -> sqlx::Result<i64> {
    let count = sqlx::query_scalar!(r#"SELECT COUNT(*) as "count!" FROM remarks"#)
        .fetch_one(pool)
        .await?;

    Ok(count)
}

pub async fn count_remarks_tags(pool: &PgPool) -> sqlx::Result<i64> {
    let count = sqlx::query_scalar!(r#"SELECT COUNT(*) as "count!" FROM remarks_tags"#)
        .fetch_one(pool)
        .await?;

    Ok(count)
}

pub async fn count_tags(pool: &PgPool) -> sqlx::Result<i64> {
    let count = sqlx::query_scalar!(r#"SELECT COUNT(*) as "count!" FROM tags"#)
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

pub async fn get_first_remark_with_tag(pool: &PgPool) -> sqlx::Result<Uuid> {
    let id = sqlx::query_scalar!(
        r#"
SELECT id
FROM remarks
JOIN remarks_tags ON remarks_tags.remark_id = remarks.id
WHERE remarks_tags.remark_id IS NOT NULL
ORDER BY created_at DESC
LIMIT 1
        "#
    )
    .fetch_one(pool)
    .await?;

    Ok(id)
}

pub async fn get_first_remark_without_tag(pool: &PgPool) -> sqlx::Result<Uuid> {
    let id = sqlx::query_scalar!(
        r#"
SELECT id
FROM remarks
LEFT JOIN remarks_tags ON remarks_tags.remark_id = remarks.id
WHERE remarks_tags.remark_id IS NULL
ORDER BY created_at DESC
LIMIT 1
        "#
    )
    .fetch_one(pool)
    .await?;

    Ok(id)
}

pub async fn get_first_tag(pool: &PgPool) -> sqlx::Result<Uuid> {
    let id = sqlx::query_scalar!(r#"SELECT id FROM tags ORDER BY created_at DESC LIMIT 1"#)
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

pub async fn get_tag_title(pool: &PgPool, id: Uuid) -> sqlx::Result<String> {
    let essence = sqlx::query_scalar!(r#"SELECT title FROM tags WHERE id = $1"#, id)
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

pub async fn remark_tag_exists(pool: &PgPool, remark_id: Uuid, tag_id: Uuid) -> sqlx::Result<bool> {
    let exists = sqlx::query_scalar!(
        r#"SELECT EXISTS(SELECT 1 FROM remarks_tags WHERE remark_id = $1 AND tag_id = $2) as "exists!""#,
        remark_id,
        tag_id
    )
    .fetch_one(pool)
    .await?;

    Ok(exists)
}

pub async fn tag_exists(pool: &PgPool, id: Uuid) -> sqlx::Result<bool> {
    let exists = sqlx::query_scalar!(
        r#"SELECT EXISTS(SELECT 1 FROM tags WHERE id = $1) as "exists!""#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(exists)
}

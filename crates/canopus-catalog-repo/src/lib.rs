use canopus_catalog::{
    Brand, BrandAttributes, BrandName, Error as CatalogError, FindOneAndDeleteBrand, InsertBrand,
    Record, Result as CatalogResult, SelectBrands,
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Repository {
    pool: sqlx::PgPool,
}

struct BrandRow {
    id: Uuid,
    name: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

struct NewBrandRow {
    id: Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Repository {
    pub async fn new(url: &str) -> eyre::Result<Self> {
        let pool = sqlx::PgPool::connect(url).await?;

        Ok(Self { pool })
    }
}

impl FindOneAndDeleteBrand for Repository {
    async fn find_one_and_delete_brand(&self, id: Uuid) -> CatalogResult<Record<Brand>> {
        let row = sqlx::query_as!(
            BrandRow,
            r#"
DELETE from brands
WHERE id = $1
RETURNING id, name, created_at, updated_at
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(from_sqlx_err)?;

        row.try_into()
    }
}

impl InsertBrand for Repository {
    async fn insert_brand(&self, data: Brand) -> CatalogResult<Record<Brand>> {
        let row = sqlx::query_as!(
            NewBrandRow,
            r#"
INSERT INTO brands (name)
VALUES ($1)
RETURNING id, created_at, updated_at
            "#,
            data.name().as_str(),
        )
        .fetch_one(&self.pool)
        .await
        .map_err(from_sqlx_err)?;

        let NewBrandRow {
            id,
            created_at,
            updated_at,
        } = row;

        Ok(Record {
            id,
            created_at,
            updated_at,
            data,
        })
    }
}

impl SelectBrands for Repository {
    async fn select_brands(&self) -> CatalogResult<Vec<Record<Brand>>> {
        let rows = sqlx::query_as!(
            BrandRow,
            r#"
SELECT id, name, created_at, updated_at
FROM brands
ORDER BY name
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(from_sqlx_err)?;

        rows.into_iter().map(TryInto::try_into).collect()
    }
}

impl TryFrom<BrandRow> for Record<Brand> {
    type Error = CatalogError;

    fn try_from(row: BrandRow) -> CatalogResult<Self> {
        let BrandRow {
            id,
            name,
            created_at,
            updated_at,
        } = row;

        Ok(Record {
            id,
            data: Brand::new(BrandAttributes {
                name: BrandName::new(name)?,
            }),
            created_at,
            updated_at,
        })
    }
}

fn from_sqlx_err(err: sqlx::Error) -> CatalogError {
    use sqlx::Error;

    match err {
        Error::RowNotFound => CatalogError::RecordNotFound,
        Error::Database(database_error) => {
            if database_error.is_unique_violation() {
                CatalogError::RecordAlreadyExists
            } else {
                CatalogError::Repo(database_error.to_string())
            }
        }
        other => CatalogError::Repo(other.to_string()),
    }
}

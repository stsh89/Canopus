use canopus_catalog::{
    Brand, BrandAttributes, BrandName, Error as CatalogError, InsertBrand, Record,
    Result as CatalogResult,
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

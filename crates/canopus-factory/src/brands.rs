use crate::{AppError, AppResult, repo::Repo};
use chrono::{DateTime, Utc};
use std::{fmt::Display, str::FromStr};
use uuid::Uuid;

const MAX_BRAND_NAME_LENGTH: usize = 100;
const MIN_BRAND_NAME_LENGTH: usize = 2;

pub struct Brand {
    id: Uuid,
    name: BrandName,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub struct CreateBrandChangeset {
    pub name: BrandName,
}

pub struct BrandAttributes {
    pub id: Uuid,
    pub name: BrandName,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct BrandName(String);

impl Brand {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn new(attributes: BrandAttributes) -> Self {
        let BrandAttributes {
            id,
            name,
            created_at,
            updated_at,
        } = attributes;

        Brand {
            id,
            name,
            created_at,
            updated_at,
        }
    }

    pub fn name(&self) -> &BrandName {
        &self.name
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

pub struct CreateBrandParameters {
    pub name: String,
}

pub async fn create_brand(repo: &Repo, parameters: CreateBrandParameters) -> AppResult<Brand> {
    let CreateBrandParameters { name } = parameters;

    let changeset = CreateBrandChangeset {
        name: name.parse()?,
    };

    insert_brand(repo, changeset)
        .await
        .map(TryInto::try_into)
        .map_err(|err| match err {
            sqlx::Error::Database(database_error) if database_error.is_unique_violation() => {
                AppError::ResourceAlreadyExists(format!(
                    "brand with the name {name} already exists"
                ))
            }
            other => other.into(),
        })?
}

pub async fn list_brands(repo: &Repo) -> AppResult<Vec<Brand>> {
    let rows = select_brands(repo).await?;

    rows.into_iter().map(TryInto::try_into).collect()
}

async fn insert_brand(repo: &Repo, changeset: CreateBrandChangeset) -> sqlx::Result<BrandRow> {
    let CreateBrandChangeset { name } = changeset;

    let mut tx = repo.transaction().await?;

    let row = sqlx::query_as!(
        BrandRow,
        r#"
INSERT INTO brands (name, slug)
VALUES ($1, lower($1))
RETURNING id, name, created_at, updated_at
        "#,
        name.as_str(),
    )
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(row)
}

async fn select_brands(repo: &Repo) -> sqlx::Result<Vec<BrandRow>> {
    let pool = repo.pool();

    sqlx::query_as!(
        BrandRow,
        r#"
SELECT id, name, created_at, updated_at
FROM brands
ORDER by name ASC
        "#
    )
    .fetch_all(pool)
    .await
}

impl BrandName {
    pub fn new(value: String) -> AppResult<Self> {
        if value.len() < MIN_BRAND_NAME_LENGTH {
            return Err(AppError::Validation(format!(
                "brand name should be at least {} characters long",
                MIN_BRAND_NAME_LENGTH
            )));
        }

        if value.len() > MAX_BRAND_NAME_LENGTH {
            return Err(AppError::Validation(format!(
                "brand name should be less or equal to {} bytes",
                MAX_BRAND_NAME_LENGTH
            )));
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub struct BrandRow {
    id: Uuid,
    name: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl TryFrom<BrandRow> for Brand {
    type Error = AppError;

    fn try_from(value: BrandRow) -> AppResult<Self> {
        let BrandRow {
            id,
            name,
            created_at,
            updated_at,
        } = value;

        Ok(Brand::new(BrandAttributes {
            id,
            name: name.parse()?,
            created_at,
            updated_at,
        }))
    }
}

impl Display for BrandName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for BrandName {
    type Err = AppError;

    fn from_str(s: &str) -> AppResult<Self> {
        Self::new(s.to_string())
    }
}

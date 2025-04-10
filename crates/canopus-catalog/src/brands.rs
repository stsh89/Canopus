use uuid::Uuid;

use crate::{Error, Record, Result};
use std::{fmt::Display, str::FromStr};

const MAX_BRAND_NAME_LENGTH: usize = 100;
const MIN_BRAND_NAME_LENGTH: usize = 2;

pub trait FindOneAndDeleteBrand {
    fn find_one_and_delete_brand(&self, id: Uuid) -> impl Future<Output = Result<Record<Brand>>>;
}

pub trait InsertBrand {
    fn insert_brand(&self, data: Brand) -> impl Future<Output = Result<Record<Brand>>>;
}

pub trait SelectBrands {
    fn select_brands(&self) -> impl Future<Output = Result<Vec<Record<Brand>>>>;
}

pub struct CreateBrand<'a, IR> {
    pub repo: &'a IR,
}

pub struct DeleteBrand<'a, FDR> {
    pub repo: &'a FDR,
}

pub struct ListBrands<'a, SR> {
    pub repo: &'a SR,
}

pub struct CreateBrandParameters {
    pub name: String,
}

pub struct Brand {
    name: BrandName,
}

pub struct BrandAttributes {
    pub name: BrandName,
}

pub struct BrandName(String);

impl Brand {
    pub fn name(&self) -> &BrandName {
        &self.name
    }

    pub fn new(attributes: BrandAttributes) -> Brand {
        let BrandAttributes { name } = attributes;

        Brand { name }
    }
}

impl BrandName {
    pub fn new(value: String) -> Result<Self> {
        if value.len() < MIN_BRAND_NAME_LENGTH {
            return Err(Error::Validation(format!(
                "brand name should be at least {} characters long",
                MIN_BRAND_NAME_LENGTH
            )));
        }

        if value.len() > MAX_BRAND_NAME_LENGTH {
            return Err(Error::Validation(format!(
                "brand name should be less or equal to {} bytes",
                MAX_BRAND_NAME_LENGTH
            )));
        }

        Ok(BrandName(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<IR> CreateBrand<'_, IR>
where
    IR: InsertBrand,
{
    pub async fn execute(&self, parameters: CreateBrandParameters) -> Result<Record<Brand>> {
        let CreateBrandParameters { name } = parameters;

        let brand = Brand::new(BrandAttributes {
            name: name.parse()?,
        });

        self.repo.insert_brand(brand).await
    }
}

impl<FDR> DeleteBrand<'_, FDR>
where
    FDR: FindOneAndDeleteBrand,
{
    pub async fn execute(&self, id: Uuid) -> Result<Record<Brand>> {
        self.repo.find_one_and_delete_brand(id).await
    }
}

impl<SB> ListBrands<'_, SB>
where
    SB: SelectBrands,
{
    pub async fn execute(&self) -> Result<Vec<Record<Brand>>> {
        self.repo.select_brands().await
    }
}

impl Display for BrandName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for BrandName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::new(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_brand_name() {
        assert!(BrandName::new("Mikrotik".to_string()).is_ok());
    }
}

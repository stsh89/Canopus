use crate::{Error, Record, Result};
use std::{fmt::Display, str::FromStr};

const MAX_BRAND_NAME_LENGTH: usize = 100;
const MIN_BRAND_NAME_LENGTH: usize = 2;

pub trait InsertBrand {
    fn insert_brand(&self, brand: Brand) -> impl Future<Output = Result<Record<Brand>>>;
}

pub struct CreateBrand<'a, IR> {
    pub repo: &'a IR,
}

pub struct CreateBrandParameters {
    name: String,
}

pub struct Brand {
    name: BrandName,
}

pub struct BrandAttributes {
    name: BrandName,
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
}

impl<'a, I> CreateBrand<'a, I>
where
    I: InsertBrand,
{
    pub async fn execute(&self, parameters: CreateBrandParameters) -> Result<Record<Brand>> {
        let CreateBrandParameters { name } = parameters;

        let brand = Brand::new(BrandAttributes {
            name: name.parse()?,
        });

        self.repo.insert_brand(brand).await
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

//! # Catalog
//!
//! Core library whose purpose is to provide basic definitions and operations
//! around network devices, primarily for collection and listing purposes.
//!  
//! ## Work in progress
//!
//! [x] Brand definition
//! [x] Create brand operation
//! [x] Delete brand operation
//! [ ] List brand operation
//!

mod brands;
mod error;

pub use brands::*;
pub use error::*;

use chrono::{DateTime, Utc};
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, Error>;

pub struct Record<T> {
    pub created_at: DateTime<Utc>,
    pub data: T,
    pub id: Uuid,
    pub updated_at: DateTime<Utc>,
}

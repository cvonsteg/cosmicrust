use std::fmt;

use crate::repo::RepoError;

use super::domain::{Batch, OrderLine, Product, DomainError};
use super::repo::Repository;

// Exceptions
type ServiceResult<T> = std::result::Result<T, ServiceError>;

#[derive(Debug, Clone)]
pub enum ServiceError {
    IO(RepoError),
    Processing(DomainError)
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IO(e) => write!(f, "IO error: {}", e),
            Self::Processing(e) => write!(f, "Processing error: {}", e)
        }
    }
}

impl From<RepoError> for ServiceError {
    fn from(value: RepoError) -> Self {
        Self::IO(value)
    }
}

impl From<DomainError> for ServiceError {
    fn from(value: DomainError) -> Self {
        Self::Processing(value)
    }
}


pub fn add_batch(batch: Batch, repo: &mut impl Repository<Product>) -> ServiceResult<()> {
    let mut product = repo.read(&batch.sku).unwrap_or(Product::new(String::from(&batch.sku), Vec::new()));
    product.append_batch(batch);
    repo.update(product)?;
    Ok(())
}


pub fn allocate(
    line: &OrderLine,
    repo: &mut impl Repository<Product>,
) -> ServiceResult<String> {
    let mut product = repo.read(&line.sku)?;
    let batch_ref = product.allocate(line)?;
    repo.update(product)?;
    Ok(batch_ref)
}

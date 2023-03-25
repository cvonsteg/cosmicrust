use std::fmt;

use super::domain::{Batch, OrderLine};
use super::repo::Repository;

// Exceptions
type AllocationResult<T> = std::result::Result<T, AllocationError>;

#[derive(Debug, Clone)]
pub struct AllocationError;

impl fmt::Display for AllocationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No allocatable batches found")
    }
}

pub fn get_closest_batch(
    line: &OrderLine,
    repo: &impl Repository<Batch, Vec<Batch>>,
) -> Option<Batch> {
    if let Ok(mut batches) = repo.read(&line.sku) {
        batches.sort();
        if let Some(first_allocatable) = batches.iter_mut().position(|x| x.can_allocate(line)) {
            let batch = &batches[first_allocatable];
            Some(batch.clone())
        } else {
            None
        }
    } else {
        None
    }
}

pub fn allocate(
    line: &OrderLine,
    repo: &mut impl Repository<Batch, Vec<Batch>>,
) -> AllocationResult<String> {
    if let Some(mut batch) = get_closest_batch(line, repo) {
        let reference = batch.reference.clone();
        batch.allocate(line);
        match repo.update(batch) {
            Ok(_) => Ok(reference),
            Err(_) => Err(AllocationError),
        }
    } else {
        Err(AllocationError)
    }
}

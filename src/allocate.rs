use std::fmt;

use super::domain::{Batch, OrderLine};


// Exceptions
type AllocationResult<T> = std::result::Result<T, AllocationError>;

#[derive(Debug, Clone)]
pub struct AllocationError;

impl fmt::Display for AllocationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No allocatable batches found")
    }
}


pub fn allocate(line: &OrderLine, mut batches: Vec<&mut Batch>) -> AllocationResult<String> {
    batches.sort();
    let first_allocatable = batches.iter_mut().position(|x| x.can_allocate(line));
    if let Some(i) = first_allocatable {
            let batch = &mut batches[i];
            batch.allocate(line);
            Ok(batch.reference.clone())
    } else {
        Err(AllocationError)
    }

    
}

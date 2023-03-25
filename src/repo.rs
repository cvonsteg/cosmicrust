use super::domain::Batch;
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone)]
pub struct ReadError;

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not read from repository")
    }
}

#[derive(Debug, Clone)]
pub struct WriteError;

impl fmt::Display for WriteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not write to repository")
    }
}

pub trait Repository<T, U> {
    fn update(&mut self, item: T) -> Result<(), WriteError>;
    fn write(&mut self, item: T) -> Result<(), WriteError>;
    fn read(&self, key: &str) -> Result<U, ReadError>;
}

pub struct LocalBatchRepo {
    batches: HashMap<String, Vec<Batch>>,
}

fn _update_map(
    batch_map: &mut HashMap<String, Vec<Batch>>,
    item: Batch,
    update: bool,
) -> Result<(), WriteError> {
    let key = item.sku.clone();
    // check the sku exists
    let batch_vec = match batch_map.get_mut(&key) {
        // if yes - check for matching ref
        Some(existing) => {
            let idx = existing.iter().position(|b| b.reference == item.reference);
            match update {
                true => {
                    // if idx exists, replace value
                    if idx.is_some() {
                        existing.remove(idx.unwrap());
                    };
                    // add new value to vec
                    existing.push(item);
                    Ok(existing.clone())
                }
                false => {
                    // if idx exists, duplicate found - err
                    if idx.is_some() {
                        Err(WriteError)
                    } else {
                        existing.push(item);
                        Ok(existing.clone())
                    }
                }
            }
        }
        // If no - update and write can both proceed
        None => {
            let new_vec = vec![item];
            // TODO: this is a hack to appease the result types of the match arms.  Feels like there should be a better way
            match new_vec.len() {
                1 => Ok(new_vec),
                _ => Err(WriteError),
            }
        }
    }?;
    batch_map.insert(key, batch_vec);
    Ok(())
}

impl Repository<Batch, Vec<Batch>> for LocalBatchRepo {
    fn write(&mut self, item: Batch) -> Result<(), WriteError> {
        _update_map(&mut self.batches, item, false)
    }
    fn update(&mut self, item: Batch) -> Result<(), WriteError> {
        _update_map(&mut self.batches, item, true)
    }

    fn read(&self, key: &str) -> Result<Vec<Batch>, ReadError> {
        let batch = self.batches.get(key);
        match batch {
            Some(b) => Ok(b.clone()),
            None => Err(ReadError),
        }
    }
}

impl LocalBatchRepo {
    pub fn new() -> Self {
        let batch_map = HashMap::new();
        LocalBatchRepo { batches: batch_map }
    }

    pub fn populate_from_vec(&mut self, batches: Vec<Batch>) {
        for batch in batches {
            self.update(batch).unwrap();
        }
    }
}

impl Default for LocalBatchRepo {
    fn default() -> Self {
        Self::new()
    }
}

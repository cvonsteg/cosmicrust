use super::domain::{Batch, Product};
use std::{collections::HashMap, fmt};


#[derive(Debug, Clone)]
pub enum RepoError {
    InvalidSku(String),
    SkuExists(String)
}


impl fmt::Display for RepoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidSku(s) => write!(f, "Invalid SKU: {}", s),
            Self::SkuExists(s) => write!(f, "SKU {} exists, must update if changes are expected", s),
        }
    }
}

pub trait Repository<T> {
    fn update(&mut self, item: T) -> Result<(), RepoError>;
    fn write(&mut self, item: T) -> Result<(), RepoError>;
    fn read(&self, key: &str) -> Result<T, RepoError>;
}

pub struct LocalBatchRepo {
    products: HashMap<String, Vec<Batch>>,
}


fn _update_map(
    products: &mut HashMap<String, Vec<Batch>>,
    item: Product, 
    update: bool
    ) -> Result<(), RepoError> {
    let sku_exists =  products.contains_key(&item.sku); 
    if !update && sku_exists {
        Err(RepoError::SkuExists(String::from(&item.sku)))
    } else {
        products.insert(item.sku, item.batches);
        Ok(())
    }
}

impl Repository<Product> for LocalBatchRepo {
    fn write(&mut self, item: Product) -> Result<(), RepoError> {
        _update_map(&mut self.products, item, false)
    }
    fn update(&mut self, item: Product) -> Result<(), RepoError> {
        _update_map(&mut self.products, item, true)
    }

    fn read(&self, key: &str) -> Result<Product, RepoError> {
        let batch = self.products.get(key);
        match batch {
            Some(b) => Ok(Product::new(String::from(key), b.clone())),
            None => Err(RepoError::InvalidSku(String::from(key))),
        }
    }
}

impl LocalBatchRepo {
    pub fn new() -> Self {
        let batch_map = HashMap::new();
        LocalBatchRepo { products: batch_map }

    }

    pub fn populate_from_vec(batches: Vec<Batch>) -> Self {
        let mut products = HashMap::<String, Vec<Batch>>::new();
        for batch in batches {
            products.entry(batch.sku.clone()).or_default().push(batch.clone());
        }
        LocalBatchRepo { products }
    }
}

impl Default for LocalBatchRepo {
    fn default() -> Self {
        Self::new()
    }
}

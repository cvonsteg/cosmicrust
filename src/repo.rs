use super::domain::Batch;
use std::collections::HashMap;

pub trait Repository<T> {
    fn add(&mut self, item: T);
    fn get(&self, key: String) -> T;
}

pub struct LocalBatchRepo {
    batches: HashMap<String, Batch>,
}

impl Repository<Batch> for LocalBatchRepo {
    fn add(&mut self, item: Batch) {
        let key = item.reference.clone();
        self.batches.insert(key, item);
    }

    fn get(&self, key: String) -> Batch {
        let batch = self.batches.get(&key);
        match batch {
            Some(b) => b.clone(),
            None => panic!("No batch found for key {:?}", key),
        }
    }
}

impl LocalBatchRepo {
    pub fn from_vec(batches: Vec<Batch>) -> LocalBatchRepo {
        let mut batch_map = HashMap::new();
        for batch in batches {
            batch_map.insert(String::from(&batch.reference), batch);
        }
        LocalBatchRepo { batches: batch_map }
    }
}

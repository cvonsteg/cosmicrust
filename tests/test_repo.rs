use cosmicrust::domain::Batch;
use cosmicrust::repo::{LocalStore, Repository};

#[test]
fn test_that_local_store_can_read_and_write_batch() {
    let batches = vec![
        Batch::new("ref1", "CHAIR", 10, None),
        Batch::new("ref2", "TABLE", 10, None),
        Batch::new("ref3", "SOFA", 10, None),
        Batch::new("ref4", "LAMP", 10, None),
    ];
    let mut local_store = LocalStore::from_vec(batches);
    let new_batch = Batch::new("ref5", "CUSHION", 10, None);

    // write
    local_store.add(new_batch);
    //read
    let result = local_store.get(String::from("ref5"));

    assert_eq!(result, Batch::new("ref5", "CUSHION", 10, None));
}

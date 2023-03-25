use chrono::Local;
use cosmicrust::domain::Batch;
use cosmicrust::repo::{LocalBatchRepo, Repository};

#[test]
fn test_that_local_store_can_read_and_write_batch() {
    let mut local_store = LocalBatchRepo::new();
    let batch = Batch::new("ref5", "CUSHION", 10, None);

    // write
    local_store.write(batch);
    //read
    let result = local_store.read("CUSHION");

    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        vec![Batch::new("ref5", "CUSHION", 10, None)]
    );
}

#[test]
fn test_returns_multiple() {
    let mut local_store = LocalBatchRepo::new();
    let batches = vec![
        Batch::new("ref1", "SKU1", 10, None),
        Batch::new("ref2", "SKU1", 10, None),
    ];
    local_store.populate_from_vec(batches);

    // when
    let result = local_store.read("SKU1");
    // then
    assert_eq!(result.unwrap().len(), 2);
}

#[test]
fn test_update_value() {
    // given
    let mut local_store = LocalBatchRepo::new();
    local_store.update(Batch::new("ref5", "CUSHION", 10, None));
    assert_eq!(
        local_store.read("CUSHION").unwrap()[0].available_quantity(),
        10
    );
    // when
    local_store.update(Batch::new("ref5", "CUSHION", 0, None));
    // then
    let readback = local_store.read("CUSHION");
    assert_eq!(readback.unwrap()[0].available_quantity(), 0);
}

#[test]
fn test_duplicate_write_raises_error() {
    // given
    let mut local_store = LocalBatchRepo::new();
    local_store.write(Batch::new("ref5", "CUSHION", 10, None));
    // when
    let result = local_store.write(Batch::new("ref5", "CUSHION", 10, None));
    // then
    assert!(result.is_err());
}

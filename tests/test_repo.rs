use chrono::Local;
use cosmicrust::domain::{Batch, Product};
use cosmicrust::repo::{LocalBatchRepo, Repository};

#[test]
fn test_that_local_store_can_read_and_write_batch() {
    let mut local_store = LocalBatchRepo::new();
    let product = Product::new(
        String::from("CUSHION"),
        vec![Batch::new("ref5", "CUSHION", 10, None)]
    );
    // write
    local_store.write(product.clone());
    //read
    let result = local_store.read("CUSHION");

    assert!(result.is_ok());
    let product_result = result.unwrap();
    assert_eq!(product_result.sku, product.sku);
    assert_eq!(product_result.batches, product.batches);
}

#[test]
fn test_returns_multiple() {
    let batches = vec![
        Batch::new("ref1", "SKU1", 10, None),
        Batch::new("ref2", "SKU1", 10, None),
    ];
    let mut local_store = LocalBatchRepo::populate_from_vec(batches);

    // when
    let result = local_store.read("SKU1");
    // then
    assert_eq!(result.unwrap().batches.len(), 2);
}

#[test]
fn test_update_value() {
    // given
    let mut local_store = LocalBatchRepo::new();
    let initial_product = Product::new(String::from("CUSHION"), vec![Batch::new("ref5", "CUSHION", 10, None)]);
    local_store.update(initial_product);
    assert_eq!(
        local_store.read("CUSHION").unwrap().batches[0].available_quantity(),
        10
    );
    let updated_product = Product::new(String::from("CUSHION"), vec![Batch::new("ref5", "CUSHION", 0, None)]);
    // when
    local_store.update(updated_product);
    // then
    let readback = local_store.read("CUSHION");
    assert_eq!(readback.unwrap().batches[0].available_quantity(), 0);
}


use chrono::{Local, NaiveDate};

use cosmicrust::domain::{Batch, OrderLine};
use cosmicrust::repo::{LocalBatchRepo, Repository};
use cosmicrust::services::{allocate, get_closest_batch};

static SKU: &str = "RETRO-CLOCK";

fn setup_repo() -> LocalBatchRepo {
    let in_stock_batch = Batch::new("in_stock_batch", SKU, 100, None);
    let shipment_batch = Batch::new(
        "shipment_batch",
        SKU,
        100,
        Some(NaiveDate::from_ymd(2022, 5, 22)),
    );
    let mut repo = LocalBatchRepo::new();
    repo.populate_from_vec(vec![in_stock_batch, shipment_batch]);
    repo
}

#[test]
fn test_get_closest_batch() {
    let repo = setup_repo();
    // when
    let should_give_batch = get_closest_batch(&OrderLine::new("oref", SKU, 10), &repo);
    let should_give_none = get_closest_batch(&OrderLine::new("oref", "DOES-NOT-EXIST", 10), &repo);
    // then
    assert!(should_give_batch.is_some());
    assert!(should_give_none.is_none());
}

#[test]
fn test_prefers_current_stock_batches_to_shipments() {
    // given
    let mut repo = setup_repo();
    let line = OrderLine::new("oref", SKU, 10);
    // when
    let result = allocate(&line, &mut repo);
    // then
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), String::from("in_stock_batch"));
}

#[test]
fn test_allocation_error_raised_if_no_batches_available() {
    // given
    let mut repo = setup_repo();
    let line = OrderLine::new("oref", "RETRO-CHAIR", 10);
    // when
    let result = allocate(&line, &mut repo);
    // then
    assert!(result.is_err(), "No allocatable batches found");
}

use chrono::{Local, NaiveDate};

use cosmicrust::domain::{Batch, OrderLine, DomainError};
use cosmicrust::repo::{LocalBatchRepo, Repository, RepoError};
use cosmicrust::services;

static SKU: &str = "RETRO-CLOCK";

fn setup_repo() -> LocalBatchRepo {
    let in_stock_batch = Batch::new("in_stock_batch", SKU, 100, None);
    let shipment_batch = Batch::new(
        "shipment_batch",
        SKU,
        100,
        Some(NaiveDate::from_ymd(2022, 5, 22)),
    );
    let repo = LocalBatchRepo::populate_from_vec(vec![in_stock_batch, shipment_batch]);
    repo
}

#[test]
fn test_add_batch_if_sku_exists(){
    // given
    let mut repo = setup_repo();
    let initial_len = repo.read(SKU).unwrap().batches.len();
    let batch = Batch::new("some_ref", SKU, 10, None);
    // when
    let result = services::add_batch(batch, &mut repo);
    // then
    assert!(result.is_ok());
    assert!(repo.read(SKU).unwrap().batches.len() > initial_len);
}

#[test]
fn test_add_batch_if_sku_doesnt_exist(){
    // given
    let mut repo = setup_repo();
    let batch = Batch::new("some_ref", "NEW-SKU", 10, None);
    // when
    let result = services::add_batch(batch, &mut repo);
    // then
    assert!(result.is_ok());
    assert!(repo.read("NEW-SKU").is_ok());
}

#[test]
fn test_successful_allocation(){
    // given
    let mut repo = setup_repo();
    let order = OrderLine::new("oref", SKU, 10);
    // when
    let result = services::allocate(&order, &mut repo);
    // then
    assert_eq!(result.unwrap(), String::from("in_stock_batch"));
}

#[test]
fn test_allocate_exceptions_caught(){
    // given
    let mut repo = setup_repo();
    let invalid_sku_order = OrderLine::new("oref", "INVALID-SKU", 10);
    let qty_too_high_order = OrderLine::new("oref", SKU, 1000);
    // when
    let invalid_sku_result = services::allocate(&invalid_sku_order, &mut repo);
    let qty_result = services::allocate(&qty_too_high_order, &mut repo);
    // then
    assert!(invalid_sku_result.is_err());
    assert!(qty_result.is_err());
}

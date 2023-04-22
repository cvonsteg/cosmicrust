use chrono::NaiveDate;

use cosmicrust::domain::{Batch, OrderLine, Product};

fn make_batch_and_line(sku: &str, batch_qty: i64, line_qty: i64) -> (Batch, OrderLine) {
    let date = NaiveDate::from_ymd(2022, 3, 26);
    let batch = Batch::new("batch-001", sku, batch_qty, Some(date));
    let order = OrderLine::new("order-123", sku, line_qty);
    (batch, order)
}

#[test]
fn test_can_allocate_if_available_greater_than_required() {
    let (large_batch, small_line) = make_batch_and_line("ELEGANT-LAMP", 20, 2);
    assert!(large_batch.can_allocate(&small_line))
}

#[test]
fn test_cannot_allocate_if_available_smaller_than_required() {
    let (small_batch, large_line) = make_batch_and_line("ELEGANT-LAMP", 2, 20);
    assert!(!small_batch.can_allocate(&large_line));
}

#[test]
fn test_can_allocate_if_available_eq_to_required() {
    let (batch, line) = make_batch_and_line("ELEGANT-LAMP", 2, 2);
    assert!(batch.can_allocate(&line));
}

#[test]
fn test_cannot_allocate_if_skus_do_not_match() {
    let batch = Batch::new("batch-001", "UNCOMFORTABLE-CHAIR", 100, None);
    let different_sku_line = OrderLine::new("order-123", "EXPENSIVE-TOASTER", 10);
    assert!(!batch.can_allocate(&different_sku_line));
}

#[test]
fn test_allocation_is_idempotent() {
    let (mut batch, line) = make_batch_and_line("ANGULAR-DESK", 20, 2);
    batch.allocate(&line);
    batch.allocate(&line);
    assert_eq!(batch.available_quantity(), 18);
}

#[test]
fn test_partial_ord_for_batches() {
    let in_stock_batch = Batch::new("in_stock_batch", "RETRO-CLOCK", 100, None);
    let shipment_batch = Batch::new(
        "shipment_batch",
        "RETRO-CLOCK",
        100,
        Some(NaiveDate::from_ymd(2022, 5, 22)),
    );
    assert!(in_stock_batch < shipment_batch);
}

#[test]
fn test_product_cannot_allocate() {
    // given
    let in_stock_sku = "in_stock_batch";
    let in_stock_batch = Batch::new("ref1", in_stock_sku, 100, None);
    let in_stock_orderline = OrderLine::new("order1", in_stock_sku, 200);
    let not_preset_orderline = OrderLine::new("order2", "not_present", 100);
    let mut product = Product::new(String::from(in_stock_sku), vec![in_stock_batch]);

    // invalid sku
    let result_not_present = product.allocate(&not_preset_orderline);
    assert!(result_not_present.is_err());
    // present but too not enough units
    let result_not_enough = product.allocate(&in_stock_orderline);
    assert!(result_not_enough.is_err());
}

#[test]
fn test_product_can_allocate() {
    // given
    let in_stock_sku = "in_stock_batch";
    let in_stock_batch = Batch::new("ref1", in_stock_sku, 100, None);
    let in_stock_orderline = OrderLine::new("order1", in_stock_sku, 20);
    let mut product = Product::new(String::from(in_stock_sku), vec![in_stock_batch]);
    // when
    let result = product.allocate(&in_stock_orderline);
    // then
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), String::from("ref1"));
}

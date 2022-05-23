use chrono::NaiveDate;

use cosmicrust::allocate::allocate;
use cosmicrust::domain::{Batch, OrderLine};


#[test]
fn test_prefers_current_stock_batches_to_shipments() {
    let mut in_stock_batch = Batch::new("in_stock_batch", "RETRO-CLOCK", 100, None);
    let mut shipment_batch = Batch::new("shipment_batch", "RETRO-CLOCK", 100, Some(NaiveDate::from_ymd(2022, 5, 22)));
    let line = OrderLine::new("oref", "RETRO-CLOCK", 10);
    let batches: Vec<&mut Batch> = vec![&mut in_stock_batch, &mut shipment_batch];

    let result = allocate(&line, batches);

    assert!(result.is_ok());

    let response = result.unwrap();

    assert_eq!(response, String::from("in_stock_batch"));
    assert_eq!(in_stock_batch.available_quantity(), 90);
    assert_eq!(shipment_batch.available_quantity(), 100);
}

#[test]
fn test_allocation_error_raised_if_no_batches_available() {
    let mut in_stock_batch = Batch::new("in_stock_batch", "RETRO-CLOCK", 100, None);
    let mut shipment_batch = Batch::new("shipment_batch", "RETRO-CLOCK", 100, Some(NaiveDate::from_ymd(2022, 5, 22)));
    let line = OrderLine::new("oref", "RETRO-CHAIR", 10);
    let batches: Vec<&mut Batch> = vec![&mut in_stock_batch, &mut shipment_batch];

    let result = allocate(&line, batches);
    
    assert!(result.is_err(), "No allocatable batches found");

}

use std::collections::HashSet;
use std::cmp::Ordering;
use std::fmt;

use chrono::NaiveDate;

type Quantity = i64;
type Sku = String;
type Reference = String;

// Exceptions
type AllocationResult<T> = std::result::Result<T, AllocationError>;

#[derive(Debug, Clone)]
pub struct AllocationError;

impl fmt::Display for AllocationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No allocatable batches found")
    }
}


#[derive(Debug, Hash, Clone)]
pub struct OrderLine {
    order_id: String,
    sku: Sku,
    qty: Quantity,
}

impl PartialEq for OrderLine {
    fn eq(&self, other: &Self) -> bool {
        (self.sku == other.sku) && (self.qty == other.qty) && (self.order_id == other.order_id)
    }
}

impl Eq for OrderLine {}

impl OrderLine {
    pub fn new(order_id: &str, sku: &str, qty: i64) -> OrderLine {
        OrderLine {
            order_id: order_id.to_owned(),
            sku: sku.to_owned(),
            qty,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Batch {
    reference: Reference,
    sku: Sku,
    eta: Option<NaiveDate>,
    _purchased_qty: Quantity,
    _allocations: HashSet<OrderLine>,
}

impl PartialEq for Batch {
    fn eq(&self, other: &Self) -> bool {
        self.sku == other.sku && self.eta == other.eta
    }
}

impl Eq for Batch {}

impl PartialOrd for Batch {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.eta.is_none(){
            Some(Ordering::Less)
        } else if other.eta.is_none() {
            Some(Ordering::Greater)
        } else {
            self.eta.partial_cmp(&other.eta)
        }
    }
}

impl Ord for Batch {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.eta.is_none(){
            Ordering::Less
        } else if other.eta.is_none() {
            Ordering::Greater
        } else {
            self.eta.cmp(&other.eta)
        }
    }
}

impl Batch {
    pub fn new(reference: &str, sku: &str, qty: i64, eta: Option<NaiveDate>) -> Batch {
        let empty_allocations: HashSet<OrderLine> = HashSet::new();
        Batch {
            reference: reference.to_owned(),
            sku: sku.to_owned(),
            _purchased_qty: qty,
            eta,
            _allocations: empty_allocations,
        }
    }

    pub fn allocate(&mut self, line: &OrderLine) {
        if self.can_allocate(line) {
            self._allocations.insert(line.clone());
        }
    }

    pub fn deallocate(&mut self, line: &OrderLine) {
        if self._allocations.contains(line) {
            self._allocations.remove(line);
        }
    }

    pub fn can_allocate(&self, line: &OrderLine) -> bool {
        let sku_match = self.sku == line.sku;
        let qty_ok = self.available_quantity() >= line.qty;
        sku_match && qty_ok
    }

    pub fn allocated_quantity(&self) -> Quantity {
        self._allocations.iter().map(|x| x.qty).sum()
    }

    pub fn available_quantity(&self) -> Quantity {
        self._purchased_qty - self.allocated_quantity()
    }
}

pub fn allocate(line: &OrderLine, mut batches: Vec<&mut Batch>) -> AllocationResult<String> {
    batches.sort();
    let first_allocatable = batches.iter_mut().position(|x| x.can_allocate(line));
    match first_allocatable {
        None => Err(AllocationError),
        Some(i) => {
            let batch = &mut batches[i];
            batch.allocate(line);
            Ok(batch.reference.clone())
        }
    }
    
}


#[cfg(test)]
mod tests {
    use super::{Batch, OrderLine, allocate};
    use chrono::NaiveDate;

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
       let shipment_batch = Batch::new("shipment_batch", "RETRO-CLOCK", 100, Some(NaiveDate::from_ymd(2022, 5, 22)));
       assert!(in_stock_batch < shipment_batch);
    }

    #[test]
    fn test_sort_vec_of_batches() {
       let mut batches = vec![
            Batch::new("shipment_batch", "RETRO-CLOCK", 100, Some(NaiveDate::from_ymd(2022, 5, 22))),
            Batch::new("in_stock_batch", "RETRO-CLOCK", 100, None)
       ];

       batches.sort();

       assert_eq!(
           batches,
           vec![
                Batch::new("in_stock_batch", "RETRO-CLOCK", 100, None),
                Batch::new("shipment_batch", "RETRO-CLOCK", 100, Some(NaiveDate::from_ymd(2022, 5, 22)))
           ]
        );
    }

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
}

use std::cmp::Ordering;
use std::collections::HashSet;

use chrono::NaiveDate;

// Custom types
type Quantity = i64;
type Sku = String;
type Reference = String;

#[derive(Debug, Hash, Clone)]
pub struct OrderLine {
    pub order_id: String,
    pub sku: Sku,
    pub qty: Quantity,
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
    pub reference: Reference,
    pub sku: Sku,
    pub eta: Option<NaiveDate>,
    purchased_qty: Quantity,
    allocations: HashSet<OrderLine>,
}

impl PartialEq for Batch {
    fn eq(&self, other: &Self) -> bool {
        self.sku == other.sku && self.eta == other.eta
    }
}

impl Eq for Batch {}

impl PartialOrd for Batch {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.eta.is_none() {
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
        if self.eta.is_none() {
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
            purchased_qty: qty,
            eta,
            allocations: empty_allocations,
        }
    }

    pub fn allocate(&mut self, line: &OrderLine) {
        if self.can_allocate(line) {
            self.allocations.insert(line.clone());
        }
    }

    pub fn deallocate(&mut self, line: &OrderLine) {
        if self.allocations.contains(line) {
            self.allocations.remove(line);
        }
    }

    pub fn can_allocate(&self, line: &OrderLine) -> bool {
        let sku_match = self.sku == line.sku;
        let qty_ok = self.available_quantity() >= line.qty;
        sku_match && qty_ok
    }

    pub fn allocated_quantity(&self) -> Quantity {
        self.allocations.iter().map(|x| x.qty).sum()
    }

    pub fn available_quantity(&self) -> Quantity {
        self.purchased_qty - self.allocated_quantity()
    }
}

#[cfg(test)]
mod tests {
    use super::Batch;
    use super::OrderLine;
    use chrono::NaiveDate;

    #[test]
    fn test_sort_vec_of_batches() {
        let mut batches = vec![
            Batch::new(
                "shipment_batch",
                "RETRO-CLOCK",
                100,
                Some(NaiveDate::from_ymd(2022, 5, 22)),
            ),
            Batch::new("in_stock_batch", "RETRO-CLOCK", 100, None),
        ];

        batches.sort();

        assert_eq!(
            batches,
            vec![
                Batch::new("in_stock_batch", "RETRO-CLOCK", 100, None),
                Batch::new(
                    "shipment_batch",
                    "RETRO-CLOCK",
                    100,
                    Some(NaiveDate::from_ymd(2022, 5, 22))
                )
            ]
        );
    }

    #[test]
    fn test_allocated_arithmetic() {
        let mut batch = Batch::new("abc123", "sku1", 100, None);
        let order_line = OrderLine::new("id1", "sku1", 10);

        assert_eq!(batch.allocated_quantity(), 0);
        assert_eq!(batch.available_quantity(), 100);

        batch.allocate(&order_line);

        assert_eq!(batch.allocated_quantity(), 10);
        assert_eq!(batch.available_quantity(), 90);
    }
}

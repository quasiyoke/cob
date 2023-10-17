use crate::payload::{Increment, Snapshot};

use part::BookPart;

mod part;

pub struct OrderBookMachine {
    bids: BookPart,
    asks: BookPart,
}

impl OrderBookMachine {
    pub fn new(s: Snapshot) -> Self {
        Self {
            bids: BookPart::new(s.bids),
            asks: BookPart::new(s.asks),
        }
    }

    pub fn apply(&mut self, inc: Increment) -> Result<(), OrderBookError> {
        self.bids.apply(inc.bids)?;
        self.asks.apply(inc.asks)?;
        if let (Some(), Some()) {}
        Ok(())
    }
}

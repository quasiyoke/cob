use std::ops::{RangeFrom, RangeTo};

use derive_more::{Display, Error};

use crate::{payload::PriceNodes, Increment, Instrument, Snapshot, Timestamp};

use part::BookPart;

mod part;

pub struct OrderBookMachine {
    bids: BookPart<RangeTo<()>>,
    asks: BookPart<RangeFrom<()>>,
    instrument: Instrument,
    timestamp: Timestamp,
}

#[derive(Clone, Debug, Eq, PartialEq, Display, Error)]
pub enum OrderBookError {
    Crossed,
    UnknownPriceNodeDrop,
}

impl OrderBookMachine {
    pub fn new(s: Snapshot) -> Result<Self, OrderBookError> {
        let s: PriceNodes = s.into();
        let instance = Self {
            bids: BookPart::new(s.bids),
            asks: BookPart::new(s.asks),
            instrument: s.instrument,
            timestamp: s.timestamp,
        };
        instance.check_invariants()?;
        Ok(instance)
    }

    pub fn apply(&mut self, inc: Increment) -> Result<(), OrderBookError> {
        let inc: PriceNodes = inc.into();
        self.bids.apply(inc.bids)?;
        self.asks.apply(inc.asks)?;
        self.timestamp = inc.timestamp;
        self.check_invariants()
    }

    pub fn snapshot(&self) -> Snapshot {
        Snapshot::new(
            self.bids.iter(),
            self.asks.iter(),
            self.instrument.clone(),
            self.timestamp.clone(),
        )
    }

    fn check_invariants(&self) -> Result<(), OrderBookError> {
        if let (Some(bid), Some(ask)) = (self.bids.best(), self.asks.best()) {
            if bid.price >= ask.price {
                return Err(OrderBookError::Crossed);
            }
        }
        Ok(())
    }
}

use derive_more::{From, Into};

use serde::{Deserialize, Serialize};

use crate::fp::{Fp, Zero};

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum Message {
    Snapshot(Snapshot),
    #[serde(rename = "Update")]
    Increment(Increment),
}

/// It's just a coincidence that `PriceNodes` struct works for both snapshot and increment.
/// This newtype prevents confusion between them.
#[derive(Deserialize, Into, Serialize)]
pub struct Snapshot(PriceNodes);

/// "Update" is an overused word.
#[derive(Deserialize, Into, Serialize)]
pub struct Increment(PriceNodes);

#[non_exhaustive]
#[derive(Deserialize, Serialize)]
pub struct PriceNodes {
    pub instrument: Instrument,
    pub bids: Vec<PriceNode>,
    pub asks: Vec<PriceNode>,
    pub timestamp: Timestamp,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct PriceNode {
    pub price: Price,
    pub size: Size,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, From, Deserialize, Serialize)]
pub struct Price(Fp);

#[derive(Clone, Copy, Debug, Eq, PartialEq, From, Deserialize, Serialize)]
pub struct Size(Fp);

#[derive(Clone, Deserialize, Serialize)]
pub struct Instrument(serde_json::Value); // TODO (issue-XXX): parse instrument

#[derive(Clone, Deserialize, Serialize)]
pub struct Timestamp(serde_json::Value); // TODO (issue-XXX): parse timestamp

impl Snapshot {
    pub fn new(
        bids: impl IntoIterator<Item = PriceNode>,
        asks: impl IntoIterator<Item = PriceNode>,
        instrument: Instrument,
        timestamp: Timestamp,
    ) -> Self {
        Self(PriceNodes {
            bids: bids.into_iter().collect(),
            asks: asks.into_iter().collect(),
            instrument,
            timestamp,
        })
    }
}

impl Zero for Size {
    const ZERO: Self = Self(Fp::ZERO);
}

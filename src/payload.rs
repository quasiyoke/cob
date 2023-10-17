use serde::{Deserialize, Serialize};

use crate::fp::Fp;

#[derive(Deserialize, Serialize)]
pub enum Message {
    Snapshot(Snapshot),
    Increment(Increment),
}

#[derive(Deserialize, Serialize)]
pub struct Snapshot {
    pub instrument: serde_json::Value,
    pub bids: Vec<PriceNode>,
    pub asks: Vec<PriceNode>,
    pub timestamp: serde_json::Value,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct PriceNode {
    pub price: Price,
    pub size: Size,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct Price(Fp);

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct Size(Fp);

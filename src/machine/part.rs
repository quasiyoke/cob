use std::collections::BTreeMap;

use crate::{Price, PriceNode, Size};

use super::OrderBookError;

pub(crate) struct BookPart {
    nodes: BTreeMap<Price, Size>,
}

impl BookPart {
    pub fn new(nodes: impl IntoIterator<Item = PriceNode>) -> Self {
        Self {
            nodes: nodes.into(),
        }
    }

    pub fn best(&self) -> Option<&PriceNode> {
        self.nodes.front()
    }

    pub fn apply(&mut self, increment: &[PriceNode]) -> Result<(), OrderBookError> {
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = PriceNode> + '_ {
        self.nodes.iter().copied()
    }
}

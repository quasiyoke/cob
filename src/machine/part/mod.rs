use std::{
    collections::{btree_map, BTreeMap},
    marker::PhantomData,
    ops::{RangeFrom, RangeTo},
};

use crate::{fp::Zero, Price, PriceNode, Size};

use super::OrderBookError;

#[cfg(test)]
mod tests;

pub(crate) struct BookPart<Order> {
    nodes: BTreeMap<Price, Size>,
    _marker: PhantomData<Order>,
}

impl<R> BookPart<R> {
    pub fn new(nodes: impl IntoIterator<Item = PriceNode>) -> Self {
        Self {
            nodes: nodes
                .into_iter()
                .map(|PriceNode { price, size }| (price, size))
                .collect(),
            _marker: Default::default(),
        }
    }

    pub fn apply(
        &mut self,
        increment: impl IntoIterator<Item = PriceNode>,
    ) -> Result<(), OrderBookError> {
        for PriceNode { price, size } in increment {
            let current = self.nodes.entry(price);
            match (current, size) {
                (btree_map::Entry::Occupied(node), Size::ZERO) => {
                    let old_size = node.remove();
                    debug_assert_ne!(old_size, Size::ZERO);
                }
                (btree_map::Entry::Occupied(mut node), size) => {
                    let old_size = node.insert(size);
                    debug_assert_ne!(old_size, Size::ZERO);
                }
                (btree_map::Entry::Vacant(_), Size::ZERO) => {
                    return Err(OrderBookError::UnknownPriceNodeDrop);
                }
                (btree_map::Entry::Vacant(vacant), size) => {
                    vacant.insert(size);
                }
            }
        }
        Ok(())
    }
}

// Asks
impl BookPart<RangeFrom<()>> {
    pub fn best(&self) -> Option<PriceNode> {
        self.nodes
            .first_key_value()
            .map(|(&price, &size)| PriceNode { price, size })
    }

    pub fn iter(&self) -> impl Iterator<Item = PriceNode> + '_ {
        self.nodes
            .iter()
            .map(|(&price, &size)| PriceNode { price, size })
    }
}

// Bids
impl BookPart<RangeTo<()>> {
    pub fn best(&self) -> Option<PriceNode> {
        self.nodes
            .last_key_value()
            .map(|(&price, &size)| PriceNode { price, size })
    }

    pub fn iter(&self) -> impl Iterator<Item = PriceNode> + '_ {
        self.nodes
            .iter()
            .rev()
            .map(|(&price, &size)| PriceNode { price, size })
    }
}

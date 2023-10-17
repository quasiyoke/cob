use std::collections::BTreeMap;

use crate::{Price, PriceNode, Size};

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
        self.nodes.first()
    }
}

use std::ops::RangeFrom;

use crate::{fp::fp, PriceNode};

use super::BookPart;

#[test]
fn adds_nodes() {
    let mut part = part();
    part.apply(
        vec![
            PriceNode {
                price: fp!(21.5),
                size: fp!(1),
            },
            PriceNode {
                price: fp!(24.1),
                size: fp!(2),
            },
        ]
        .into_iter(),
    )
    .unwrap();
    let actual: Vec<_> = part.iter().collect();
    assert_eq!(
        actual,
        &[
            PriceNode {
                price: fp!(21.5),
                size: fp!(1),
            },
            PriceNode {
                price: fp!(23.7),
                size: fp!(4),
            },
            PriceNode {
                price: fp!(24.1),
                size: fp!(2),
            },
            PriceNode {
                price: fp!(24.8),
                size: fp!(5),
            },
            PriceNode {
                price: fp!(25.9),
                size: fp!(3),
            },
        ],
    );
}

#[test]
fn updates_sizes() {
    let mut part = part();
    part.apply(vec![
        PriceNode {
            price: fp!(23.7),
            size: fp!(1),
        },
        PriceNode {
            price: fp!(24.8),
            size: fp!(2),
        },
    ])
    .unwrap();
    let actual: Vec<_> = part.iter().collect();
    assert_eq!(
        actual,
        &[
            PriceNode {
                price: fp!(23.7),
                size: fp!(1),
            },
            PriceNode {
                price: fp!(24.8),
                size: fp!(2),
            },
            PriceNode {
                price: fp!(25.9),
                size: fp!(3),
            },
        ],
    );
}

#[test]
fn on_increment_with_zero_sizes_drops_nodes() {
    let mut part = part();
    part.apply(vec![
        PriceNode {
            price: fp!(23.7),
            size: fp!(0),
        },
        PriceNode {
            price: fp!(24.8),
            size: fp!(0),
        },
    ])
    .unwrap();
    let actual: Vec<_> = part.iter().collect();
    assert_eq!(
        actual,
        &[PriceNode {
            price: fp!(25.9),
            size: fp!(3),
        }],
    );
}

#[test]
fn on_unknown_node_with_zero_size_errs() {
    assert!(part()
        .apply(vec![PriceNode {
            price: fp!(22.7),
            size: fp!(0),
        },])
        .is_err());
}

fn part() -> BookPart<RangeFrom<()>> {
    BookPart::new(vec![
        PriceNode {
            price: fp!(23.7),
            size: fp!(4),
        },
        PriceNode {
            price: fp!(24.8),
            size: fp!(5),
        },
        PriceNode {
            price: fp!(25.9),
            size: fp!(3),
        },
    ])
}

use derive_more::{Display, Error};
use futures::{Stream, StreamExt};
use stream_generator::generate_try_stream;

use crate::{Message, OrderBookError, OrderBookMachine};

#[derive(Clone, Debug, Eq, PartialEq, Display, Error)]
pub enum OrderBookStreamError {
    CantSerializeSnapshot,
    Interrupted,
    InvalidMessage,
    MissingInitialSnapshot,
    InvalidSnapshot(OrderBookError),
    InvalidIncrement(OrderBookError),
}

pub fn snapshots(
    mut messages: impl Stream<Item = Vec<u8>> + Unpin,
) -> impl Stream<Item = Result<Vec<u8>, OrderBookStreamError>> {
    generate_try_stream(|mut yielder| async move {
        let mut order_book = None;
        while let Some(bytes) = messages.next().await {
            let message = serde_json::from_slice(&bytes)
                .map_err(|_err| OrderBookStreamError::InvalidMessage)?;
            let output = match message {
                Message::Snapshot(s) => {
                    order_book = Some(
                        OrderBookMachine::new(s).map_err(OrderBookStreamError::InvalidSnapshot)?,
                    );
                    bytes
                }
                Message::Increment(inc) => {
                    let order_book = order_book
                        .as_mut()
                        .ok_or(OrderBookStreamError::MissingInitialSnapshot)?;
                    order_book
                        .apply(inc)
                        .map_err(OrderBookStreamError::InvalidIncrement)?;
                    let s = order_book.snapshot();
                    // Yielder accepts `Result` but we need to interrupt stream after sending an error.
                    serde_json::to_vec(&Message::Snapshot(s))
                        .map_err(|_err| OrderBookStreamError::CantSerializeSnapshot)?
                }
            };
            yielder.send(Ok(output)).await;
        }
        Err(OrderBookStreamError::Interrupted)
    })
}

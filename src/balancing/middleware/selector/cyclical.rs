//! Round-robin selection algorithm.

use super::Selector;
use async_trait::async_trait;
use std::net::SocketAddr;
use thiserror::Error;
use tokio::sync::{mpsc, oneshot};

/// A round-robin selector.
#[derive(Debug)]
pub struct RoundRobin {
    msgs: mpsc::Sender<Message>,
}

/// A request to select a backend.
#[derive(Debug)]
pub struct Message {
    cb: oneshot::Sender<Option<SocketAddr>>,
}

impl RoundRobin {
    /// Creates a round-robin selector for a backend within the provided sequence.
    ///
    /// Backends are selected cyclically in-order.
    pub fn with_backends(backends: Vec<SocketAddr>) -> RoundRobin {
        let (tx, rx) = mpsc::channel(1024); // TODO: Read from config.

        tokio::spawn(handle_messages(backends, rx));

        Self { msgs: tx }
    }
}

#[async_trait]
impl Selector for RoundRobin {
    type Error = RoundRobinError;

    async fn select(&self) -> Result<Option<SocketAddr>, Self::Error> {
        let (tx, rx) = oneshot::channel();

        self.msgs
            .send(Message { cb: tx })
            .await
            .map_err(RoundRobinError::RequestBackendInfo)?;

        rx.await.map_err(RoundRobinError::ReceiveBackendInfo)
    }
}

#[derive(Debug, Error)]
pub enum RoundRobinError {
    #[error("unable to request backend info")]
    RequestBackendInfo(#[source] mpsc::error::SendError<Message>),
    #[error("unable to receive backend info")]
    ReceiveBackendInfo(#[source] oneshot::error::RecvError),
}

async fn handle_messages(backends: Vec<SocketAddr>, mut rx: mpsc::Receiver<Message>) {
    let mut backends = backends.into_iter().cycle();
    while let Some(msg) = rx.recv().await {
        let _ = msg.cb.send(backends.next());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{stream, StreamExt};
    use std::net::Ipv4Addr;

    #[tokio::test]
    async fn select_returns_no_backend_when_an_empty_sequence_was_provided() {
        // Pre-condition.
        let backends = vec![];

        // Action.
        let round_robin = RoundRobin::with_backends(backends);

        let selected_backend = round_robin.select().await.unwrap();

        // Post-condition.
        assert_eq!(selected_backend, None)
    }

    #[tokio::test]
    async fn select_returns_backends_cyclically() {
        // Pre-condition.
        let backend_a = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 70);
        let backend_b = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 80);
        let backend_c = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 90);

        let backends = vec![backend_a, backend_b, backend_c];

        // Action.
        let round_robin = RoundRobin::with_backends(backends);

        let selected_backends = stream::iter(0..6)
            .then(|_| round_robin.select())
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .filter_map(Result::ok)
            .flatten()
            .collect::<Vec<_>>();

        // Post-condition.
        assert_eq!(
            selected_backends,
            vec![backend_a, backend_b, backend_c, backend_a, backend_b, backend_c]
        )
    }
}

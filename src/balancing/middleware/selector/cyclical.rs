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
    cb: oneshot::Sender<SocketAddr>,
}

impl RoundRobin {
    /// Creates a round-robin selector relevant and handle.
    ///
    /// Backends are taken from the given list.
    pub fn new(backends: Vec<SocketAddr>) -> RoundRobin {
        let (tx, rx) = mpsc::channel(1024); // TODO: Read from config.

        tokio::spawn(handle_messages(backends, rx));

        Self { msgs: tx }
    }
}

#[async_trait]
impl Selector for RoundRobin {
    type Error = RoundRobinError;

    async fn select(&self) -> Result<SocketAddr, Self::Error> {
        let (tx, rx) = oneshot::channel();

        self.msgs.send(Message { cb: tx }).await?;

        Ok(rx.await?)
    }
}

#[derive(Debug, Error)]
pub enum RoundRobinError {
    #[error("unable to request backend info")]
    RequestBackendInfo(#[from] mpsc::error::SendError<Message>),
    #[error("unable to receive backend info")]
    ReceiveBackendInfo(#[from] oneshot::error::RecvError),
}

async fn handle_messages(backends: Vec<SocketAddr>, mut rx: mpsc::Receiver<Message>) {
    let mut backends = backends.into_iter().cycle();
    while let Some(msg) = rx.recv().await {
        let backend = backends.next().unwrap();
        let _ = msg.cb.send(backend);
    }
}

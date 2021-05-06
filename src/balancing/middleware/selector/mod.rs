//! Routing of incoming connections to an appropriate backend.

use async_trait::async_trait;
use std::net::SocketAddr;

pub mod cyclical;

/// Selector for a target backend to server an incoming connection.
#[async_trait]
pub trait Selector {
    /// Error due to the selection operation.
    type Error;

    /// Selects an appropriate backend.
    ///
    /// Given an incoming connection, selects which backend should handle it.
    async fn select(&self) -> Result<SocketAddr, Self::Error>;
}

//! Opens a Tcp client connection.

use super::{Pipe, PipeError};
use anyhow::Context;
use async_trait::async_trait;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tracing::{debug, info, Level};

/// A client connector.
#[derive(Debug)]
pub struct Client;

/// Constructs a new [Client].
pub fn connect() -> Client {
    Client
}

#[async_trait]
impl Pipe<SocketAddr> for Client {
    type Output = TcpStream;

    async fn through(&self, target_address: SocketAddr) -> Result<Self::Output, PipeError> {
        let span = tracing::span!(Level::DEBUG, "client_connect", target_address = %target_address);
        let _enter = span.enter();

        debug!("opening client connection");

        let conn = TcpStream::connect(target_address)
            .await
            .with_context(|| format!("unable to open connection to {}", target_address))?;

        info!("opened client connection");

        Ok(conn)
    }
}

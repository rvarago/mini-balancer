//! Handler for frontend connections.

use crate::config;
use anyhow::Context;
use std::net::SocketAddr;
use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::{TcpListener, TcpStream},
};
use tracing::{error, info};

/// A frontend server.
pub struct Frontend {
    name: config::FrontendName,
    config: config::Frontend,
}

impl Frontend {
    /// Constructs a new [Frontend].
    pub fn new(name: config::FrontendName, config: config::Frontend) -> Self {
        Self { name, config }
    }

    pub async fn serve(self, local_address: SocketAddr) -> anyhow::Result<()> {
        let mut listener = self.bind(local_address).await?;
        loop {
            let (stream, peer_address) = self.accept_from(&mut listener).await?;
            tokio::spawn(async move {
                info!("serving connection from {}", peer_address);
                if let Err(e) = handle_connection(stream, peer_address).await {
                    error!("served connection from {}: {}", peer_address, e)
                } else {
                    info!("served connection from {}", peer_address)
                }
            });
        }
    }

    async fn bind(&self, local_address: SocketAddr) -> anyhow::Result<TcpListener> {
        TcpListener::bind(local_address)
            .await
            .with_context(|| format!("unable to bind {} on {}", self.name, local_address))
    }

    async fn accept_from(
        &self,
        listener: &mut TcpListener,
    ) -> anyhow::Result<(TcpStream, SocketAddr)> {
        listener
            .accept()
            .await
            .with_context(|| format!("unable to accept connection by {} ", self.name))
    }
}

async fn handle_connection<S>(stream: S, peer_address: SocketAddr) -> anyhow::Result<()>
where
    S: AsyncRead + AsyncWrite,
{
    todo!()
}

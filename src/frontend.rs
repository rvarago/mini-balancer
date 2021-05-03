//! Handler for frontend connections.

use anyhow::Context;
use std::net::SocketAddr;
use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::{TcpListener, TcpStream},
};
use tracing::{error, info};

/// A frontend server.
pub struct Server {
    local_address: SocketAddr,
}

impl Server {
    /// Constructs a new [Server].
    pub fn bind_on(local_address: SocketAddr) -> Self {
        Self { local_address }
    }

    /// Serves requests at `local_address`.
    pub async fn serve(self) -> anyhow::Result<()> {
        let mut listener = self.bind().await?;

        info!(local_address = %self.local_address, "listening for connections");

        loop {
            let (stream, peer_address) = accept_from(&mut listener).await?;
            tokio::spawn(handle_connection(stream, peer_address));
        }
    }

    async fn bind(&self) -> anyhow::Result<TcpListener> {
        TcpListener::bind(self.local_address)
            .await
            .with_context(|| format!("unable to bind on {}", self.local_address))
    }
}

async fn accept_from(listener: &mut TcpListener) -> anyhow::Result<(TcpStream, SocketAddr)> {
    listener
        .accept()
        .await
        .context("unable to accept connection")
}

#[tracing::instrument(skip(stream))]
async fn handle_connection<S>(stream: S, peer_address: SocketAddr)
where
    S: AsyncRead + AsyncWrite,
{
    info!("serving connection");
    if let Err(e) = process(stream).await {
        error!(reason = %e, "failed to served connection")
    } else {
        info!("served connection")
    }
}

async fn process<S>(_stream: S) -> anyhow::Result<()>
where
    S: AsyncRead + AsyncWrite,
{
    Ok(())
}

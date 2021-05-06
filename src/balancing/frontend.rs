//! Handler for frontend connections.

use super::middleware::Pipe;
use anyhow::Context;
use std::{net::SocketAddr, sync::Arc};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::{TcpListener, TcpStream},
};
use tracing::{error, info, Level};

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
    pub async fn serve<P>(self, middleware: P) -> anyhow::Result<()>
    where
        P: Pipe<TcpStream> + Send + Sync + 'static,
    {
        let mut listener = self.bind().await?;

        let span = tracing::span!(Level::INFO, "frontend", local_address = %self.local_address);
        let _enter = span.enter();

        let middleware = Arc::new(middleware);

        info!("listening for connections");

        while let Ok((stream, peer_address)) = accept_from(&mut listener).await {
            tokio::spawn(handle_connection(stream, peer_address, middleware.clone()));
        }

        Ok(())
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

#[tracing::instrument(skip(stream, middleware))]
async fn handle_connection<P, S>(stream: S, peer_address: SocketAddr, middleware: Arc<P>)
where
    P: Pipe<S>,
    S: AsyncRead + AsyncWrite,
{
    info!("serving connection");
    if let Err(e) = middleware.through(stream).await {
        error!(reason = %e, "failed to served connection")
    } else {
        info!("served connection")
    }
}

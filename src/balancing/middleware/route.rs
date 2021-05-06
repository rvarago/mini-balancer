//! Decides which backend should handle an incoming connection.

use super::{selector, Pipe, PipeError};
use anyhow::Context;
use async_trait::async_trait;
use std::net::SocketAddr;
use tracing::trace;

/// A backend selector.
#[derive(Debug)]
pub struct Router<R> {
    selector: R,
}

/// Constructs a new [Router<R>].
pub fn route<R>(selector: R) -> Router<R> {
    Router { selector }
}

#[async_trait]
impl<R, S> Pipe<S> for Router<R>
where
    R: selector::Selector + Send + Sync + 'static,
    R::Error: std::error::Error + Send + Sync,
    S: Send + 'static,
{
    type Output = (S, SocketAddr);

    async fn through(&self, src: S) -> Result<Self::Output, PipeError> {
        trace!("selecting a backend");

        let backend = self
            .selector
            .select()
            .await
            .context("unable to select a backend")?;

        trace!("selected a backend");

        Ok((src, backend))
    }
}

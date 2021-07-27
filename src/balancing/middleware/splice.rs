//! Splices traffic between two streams.

use super::{Pipe, PipeError};
use anyhow::Context;
use async_trait::async_trait;
use tokio::io::{copy_bidirectional, AsyncRead, AsyncWrite};
use tracing::trace;

/// A splicer for traffic between two streams.
#[derive(Debug)]
pub struct Splice;

/// Constructs a new [Splice].
pub fn splice() -> Splice {
    Splice
}

#[async_trait]
impl<Sl, Sr> Pipe<(Sl, Sr)> for Splice
where
    Sl: AsyncRead + AsyncWrite + Send + Unpin + 'static,
    Sr: AsyncRead + AsyncWrite + Send + Unpin + 'static,
{
    type Output = ();

    async fn through(&self, src: (Sl, Sr)) -> Result<Self::Output, PipeError> {
        trace!("splicing streams");

        let mut inbound = src.0;
        let mut outbound = src.1;

        copy_bidirectional(&mut inbound, &mut outbound)
            .await
            .context("unable to splice streams")?;

        trace!("spliced streams");

        Ok(())
    }
}

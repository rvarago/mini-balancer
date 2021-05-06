//! Splices traffic between two streams.

use super::{Pipe, PipeError};
use anyhow::Context;
use async_trait::async_trait;
use tokio::io::{self, AsyncRead, AsyncWrite, AsyncWriteExt};
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
    Sl: AsyncRead + AsyncWrite + Send + 'static,
    Sr: AsyncRead + AsyncWrite + Send + 'static,
{
    type Output = ();

    async fn through(&self, src: (Sl, Sr)) -> Result<Self::Output, PipeError> {
        trace!("splicing streams");

        let (mut ri, mut wi) = io::split(src.0);
        let (mut ro, mut wo) = io::split(src.1);

        tokio::try_join!(
            async {
                io::copy(&mut ri, &mut wo).await?;
                wo.shutdown().await
            },
            async {
                io::copy(&mut ro, &mut wi).await?;
                wi.shutdown().await
            }
        )
        .context("unable to splice streams")?;

        trace!("spliced streams");

        Ok(())
    }
}

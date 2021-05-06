//! Chains two pipes sequentially.

use super::super::{Pipe, PipeError};
use async_trait::async_trait;

/// A chain of two pipes.
#[derive(Debug)]
pub struct Chain<PipeL, PipeR> {
    left: PipeL,
    right: PipeR,
}

/// Constructs a new [Chain].
pub fn chain<PipeL, PipeR>(left: PipeL, right: PipeR) -> Chain<PipeL, PipeR> {
    Chain { left, right }
}

#[async_trait]
impl<PipeL, PipeR, S> Pipe<S> for Chain<PipeL, PipeR>
where
    PipeL: Pipe<S> + Send + Sync,
    PipeL::Output: Send,
    PipeR: Pipe<PipeL::Output> + Send + Sync,
    S: Send + 'static,
{
    type Output = PipeR::Output;

    async fn through(&self, src: S) -> Result<Self::Output, PipeError> {
        let src = self.left.through(src).await?;
        self.right.through(src).await
    }
}

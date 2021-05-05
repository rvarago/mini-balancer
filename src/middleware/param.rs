//! Exposes a static parameter.

use super::{Pipe, PipeError};
use async_trait::async_trait;
use tracing::trace;

/// A static parameter
#[derive(Debug)]
pub struct Param<T> {
    value: T,
}

/// Constructs a new [Param].
pub fn param<T>(value: T) -> Param<T> {
    Param { value }
}

#[async_trait]
impl<T, S> Pipe<S> for Param<T>
where
    T: Clone + Send + Sync + 'static,
    S: Send + 'static,
{
    type Output = (S, T);

    async fn through(&self, src: S) -> Result<Self::Output, PipeError> {
        trace!("exposing static parameter");

        Ok((src, self.value.clone()))
    }
}

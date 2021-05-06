//! Projections of a pipe of tuples into a pipe of a component.

use super::{Pipe, PipeError};
use async_trait::async_trait;

/// A projection of a pipe (S, X) into a pipe of S.
#[derive(Debug)]
pub struct First<P> {
    inner: P,
}

/// Constructs a new [First].
pub fn fst<P>(pipe: P) -> First<P> {
    First { inner: pipe }
}

#[async_trait]
impl<P, S, X> Pipe<(S, X)> for First<P>
where
    P: Pipe<S> + Send + Sync,
    S: Send + 'static,
    X: Send + 'static,
{
    type Output = (P::Output, X);

    async fn through(&self, src: (S, X)) -> Result<Self::Output, PipeError> {
        let res = self.inner.through(src.0).await?;
        Ok((res, src.1))
    }
}

/// A projection of a pipe (X, S) into a pipe of S.
#[derive(Debug)]
pub struct Second<P> {
    inner: P,
}

/// Constructs a new [Second].
pub fn snd<P>(pipe: P) -> Second<P> {
    Second { inner: pipe }
}

#[async_trait]
impl<P, S, X> Pipe<(X, S)> for Second<P>
where
    P: Pipe<S> + Send + Sync,
    S: Send + 'static,
    X: Send + 'static,
{
    type Output = (X, P::Output);

    async fn through(&self, src: (X, S)) -> Result<Self::Output, PipeError> {
        let res = self.inner.through(src.1).await?;
        Ok((src.0, res))
    }
}

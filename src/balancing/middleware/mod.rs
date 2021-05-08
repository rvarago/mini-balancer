//! A collection of middleware built for composability.

pub mod adapters;
pub mod client;
pub mod route;
pub mod selector;
pub mod splice;

use self::adapters::{
    chain::{chain, Chain},
    proj,
};
use async_trait::async_trait;
use std::marker::PhantomData;

/// An abstraction over a middleware, where a source gets processed.
#[async_trait]
pub trait Pipe<Stream> {
    /// Return type.
    type Output;

    /// Transform `src` into a fallible output.
    async fn through(&self, src: Stream) -> Result<Self::Output, PipeError>;
}

pub(super) type PipeError = anyhow::Error;

/// Fluent Api to build pipes.
#[derive(Debug)]
pub struct PipeBuilder<P, S> {
    pipe: P,
    _source: PhantomData<S>,
}

impl<P, S> PipeBuilder<P, S>
where
    P: Pipe<S>,
{
    pub fn with(pipe: P) -> Self {
        Self::new(pipe)
    }

    pub fn chain<R>(self, then: R) -> PipeBuilder<Chain<P, R>, S> {
        PipeBuilder::new(chain(self.pipe, then))
    }

    pub fn chain_snd<R>(self, then: R) -> PipeBuilder<Chain<P, proj::Second<R>>, S> {
        self.chain(proj::snd(then))
    }

    pub fn build(self) -> impl Pipe<S> {
        self.pipe
    }
}

impl<P, S> PipeBuilder<P, S> {
    fn new(pipe: P) -> Self {
        Self {
            pipe,
            _source: PhantomData,
        }
    }
}

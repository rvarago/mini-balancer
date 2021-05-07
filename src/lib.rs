//! Application wiring and startup routines.

pub mod config;
pub mod trace;

mod balancing;

use crate::balancing::{connect, route, splice, PipeBuilder, RoundRobin, Server};

/// Starts up the application.
pub async fn start_with(config: config::App) -> anyhow::Result<()> {
    start_frontend_with(config.frontend).await
}

async fn start_frontend_with(config: config::Frontend) -> anyhow::Result<()> {
    let round_robin = RoundRobin::new(
        config
            .backends
            .into_iter()
            .map(|backend| backend.target_address)
            .collect(),
    );

    let middleware = PipeBuilder::with(route(round_robin))
        .chain_snd(connect())
        .chain(splice())
        .build();

    Server::bind_on(config.local_address)
        .serve(middleware)
        .await
}

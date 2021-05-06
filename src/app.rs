//! Application wiring and startup routines.

use crate::{
    balancing::{connect, route, splice, PipeBuilder, RoundRobin, Server},
    config,
};

/// Starts up the application.
pub async fn start_with(config: config::App) -> anyhow::Result<()> {
    let round_robin = RoundRobin::new(
        config
            .frontend
            .backends
            .into_iter()
            .map(|backend| backend.target_address)
            .collect(),
    );

    let middleware = PipeBuilder::with(route(round_robin))
        .chain_snd(connect())
        .chain(splice())
        .build();

    Server::bind_on(config.frontend.local_address)
        .serve(middleware)
        .await
}

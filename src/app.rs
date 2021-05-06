//! Application wiring and startup routines.

use crate::{
    balancing::{connect, param, splice, PipeBuilder, Server},
    config,
};

/// Starts up the application.
pub async fn start_with(config: config::App) -> anyhow::Result<()> {
    let target_address = config.frontend.backends.first().unwrap().target_address;

    let middleware = PipeBuilder::with(param(target_address))
        .chain_snd(connect())
        .chain(splice())
        .build();

    Server::bind_on(config.frontend.local_address)
        .serve(middleware)
        .await
}

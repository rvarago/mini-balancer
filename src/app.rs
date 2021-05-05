//! Application wiring and startup routines.

use crate::{
    config, frontend,
    middleware::{connect, param, splice, PipeBuilder},
};

/// Starts up the application.
pub async fn start_with(config: config::App) -> anyhow::Result<()> {
    let target_address = config.frontend.backends.first().unwrap().target_address;

    let middleware = PipeBuilder::with(param(target_address))
        .chain_snd(connect())
        .chain(splice())
        .build();

    frontend::Server::bind_on(config.frontend.local_address)
        .serve(middleware)
        .await
}

//! Application wiring and startup routines.

use crate::{config, frontend};

/// Starts up the application.
pub async fn start_with(config: config::App) -> anyhow::Result<()> {
    frontend::Server::bind_on(config.frontend.local_address)
        .serve()
        .await
}

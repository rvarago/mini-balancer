//! A mini load balancer.

use anyhow::Context;
use mini_balancer::{config, trace};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

/// An interface to configure the application's general behaviour.
#[derive(Debug, StructOpt)]
#[structopt(name = "mini-balancer")]
struct Cli {
    #[structopt(
        short,
        long,
        parse(from_os_str),
        default_value = "mini-balancer.toml",
        help = "Sets the path to the configuration file"
    )]
    config: PathBuf,

    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::from_args();
    run(cli).await
}

async fn run(cli: Cli) -> anyhow::Result<()> {
    trace::init(cli.verbose.into());
    let config = read_config(&cli.config).await?;
    mini_balancer::start_with(config).await
}

async fn read_config<S>(path: S) -> anyhow::Result<config::App>
where
    S: AsRef<Path>,
{
    let content = tokio::fs::read_to_string(path.as_ref())
        .await
        .with_context(|| {
            format!(
                "failed to read configuration at {}",
                path.as_ref().to_string_lossy().as_ref()
            )
        })?;

    config::App::from_toml(content.as_str()).context("failed to parse configuration")
}

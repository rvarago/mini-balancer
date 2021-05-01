mod config;
mod trace;

use anyhow::Context;
use config::AppConfig;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "mini-balancer")]
struct Cli {
    #[structopt(short, long, parse(from_os_str), default_value = "mini-balancer.toml")]
    config: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::from_args();
    trace::init();
    run(cli).await
}

async fn run(cli: Cli) -> anyhow::Result<()> {
    let config = read_config(&cli.config).await?;
    todo!()
}

async fn read_config<S>(path: S) -> anyhow::Result<AppConfig>
where
    S: AsRef<Path>,
{
    let content = tokio::fs::read_to_string(path.as_ref())
        .await
        .with_context(|| {
            format!(
                "Failed to read configuration at {}",
                path.as_ref().to_string_lossy().as_ref()
            )
        })?;

    AppConfig::from_toml(content.as_str()).context("Failed to parse configuration")
}

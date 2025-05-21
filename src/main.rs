mod command;
mod controller;
mod tcp_server;

use crate::controller::FiringController;
use crate::tcp_server::start_tcp_server;
use anyhow::Result;
use clap::Parser;
use tracing::info;

#[derive(Parser, Debug)]
#[command(name = "fire-control")]
#[command(about = "Satellite propulsion firing controller", version)]
struct Cli {
    /// Port to listen on
    #[arg(short, long, default_value_t = 8124)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    // ---
    init_tracing();

    let cli = Cli::parse();
    info!(port = cli.port, "Starting fire-control...");

    let controller = FiringController::new();
    start_tcp_server(cli.port, controller).await?;

    Ok(())
}

fn init_tracing() {
    use tracing_subscriber::{fmt, EnvFilter};

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn")); // fallback to warn

    fmt().with_env_filter(filter).with_target(false).init();
}

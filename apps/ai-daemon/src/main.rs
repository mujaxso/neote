//! AI daemon for Zaroxi Studio.

use tokio::signal;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize tracing
    // tracing_subscriber::fmt::init();

    info!("Starting AI daemon");

    // TODO: Initialize AI agent
    // TODO: Start RPC server

    info!("AI daemon running");

    // Wait for shutdown signal
    signal::ctrl_c().await?;
    info!("Shutting down AI daemon");

    Ok(())
}

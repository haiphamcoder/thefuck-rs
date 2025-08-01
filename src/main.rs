use clap::Parser;
use thefuck_rs::{TheFuckResult, cli::Cli};

#[tokio::main]
#[allow(clippy::type_complexity)]
async fn main() -> TheFuckResult<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Parse command line arguments
    let cli = Cli::parse();

    // Run the application
    thefuck_rs::run(cli).await?;

    Ok(())
}

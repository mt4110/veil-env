mod cli;
mod fetcher;
mod process;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing::Level::WARN)
        .without_time()
        .with_ansi(false)
        .init();
    let code = match execute(Cli::parse()).await {
        Ok(code) => code,
        Err(error) => {
            tracing::error!("{error:#}");
            1
        }
    };
    std::process::exit(code);
}

async fn execute(cli: Cli) -> Result<i32> {
    match cli.command {
        Commands::Run {
            source,
            clear_env,
            command,
        } => {
            let secrets = fetcher::fetch_secrets(&source).await?;
            // Fetch completes first; no concurrent async tasks run during the blocking wait.
            process::run(&command, secrets, clear_env)
        }
    }
}

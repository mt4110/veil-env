use std::ffi::OsString;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run a command with injected environment variables (mock only).
    Run {
        /// Secret source. Only mock:demo is implemented.
        #[arg(long)]
        source: String,
        /// Clear the inherited environment before injection.
        #[arg(long)]
        clear_env: bool,
        /// Executable and arguments, following --. No shell is added.
        #[arg(required = true, last = true, num_args = 1..)]
        command: Vec<OsString>,
    },
}

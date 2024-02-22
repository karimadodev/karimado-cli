mod new;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::cli::new::NewCommand;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New(NewCommand),
}

pub fn execute() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::New(cmd) => cmd.execute(),
    }
}

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::cli::new::NewCommand;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct RootCommand {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New(NewCommand),
}

pub fn execute() -> Result<()> {
    let cli = RootCommand::parse();
    match &cli.command {
        Commands::New(cmd) => NewCommand::execute(cmd),
    }
}

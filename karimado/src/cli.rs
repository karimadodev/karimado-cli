mod contrib;
mod new;
mod run;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None, styles = contrib::styles())]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Karimado application at the path you specify
    New(new::NewCommand),

    /// Run a defined task
    Run(run::RunCommand),
}

pub fn execute() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::New(cmd) => cmd.execute(),
        Commands::Run(cmd) => cmd.execute(),
    }
}

mod build;
mod new;
mod run;
mod scaffold;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::contrib;

#[derive(Parser)]
#[command(version, about, long_about = None, disable_help_subcommand = true, styles = contrib::clap::styles())]
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

    /// Parse and transform Karimado Modeling Language files
    Build(build::BuildCommand),

    /// Install scaffolding files in a specific location
    #[command(name = "scaffold:install")]
    ScaffoldInstall(scaffold::InstallCommand),
}

pub fn execute() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::New(cmd) => cmd.execute(),
        Commands::Run(cmd) => cmd.execute(),
        Commands::Build(cmd) => cmd.execute(),
        Commands::ScaffoldInstall(cmd) => cmd.execute(),
    }
}

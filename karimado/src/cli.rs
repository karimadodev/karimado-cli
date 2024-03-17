mod build;
mod new;
mod run;
mod scaffold;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use colored::Colorize;

use crate::contrib;

#[derive(Parser)]
#[command(version, about, long_about = None, disable_help_subcommand = true, styles = contrib::clap::styles())]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
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

pub fn execute() -> i32 {
    let cli = Cli::parse();

    let level_filter = cli.verbose.log_level_filter();
    let debugging = level_filter >= log::LevelFilter::Debug;
    let timestamp: Option<env_logger::fmt::TimestampPrecision> = if debugging {
        Some(env_logger::fmt::TimestampPrecision::Seconds)
    } else {
        None
    };
    env_logger::builder()
        .format_level(debugging)
        .format_target(false)
        .format_timestamp(timestamp)
        .filter_level(level_filter)
        .init();

    if let Err(err) = match &cli.command {
        Commands::New(cmd) => cmd.execute(),
        Commands::Run(cmd) => cmd.execute(),
        Commands::Build(cmd) => cmd.execute(),
        Commands::ScaffoldInstall(cmd) => cmd.execute(),
    } {
        log::error!("{}", format!("{:?}", err).red());
        1
    } else {
        0
    }
}

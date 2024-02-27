use anyhow::Result;
use clap::Args;
use std::env;

use crate::cli::contrib;

#[derive(Args)]
pub(crate) struct BuildCommand {
    /// Watch input files
    #[arg(long)]
    watch: bool,
}

impl BuildCommand {
    pub(crate) fn execute(&self) -> Result<()> {
        let path = contrib::root_path()?;
        env::set_current_dir(&path)?;

        Ok(())
    }
}

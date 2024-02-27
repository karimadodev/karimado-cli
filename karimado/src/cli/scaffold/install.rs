use anyhow::Result;
use clap::Args;
use std::env;

use crate::cli::contrib;

#[derive(Args)]
pub(crate) struct InstallCommand {}

impl InstallCommand {
    pub(crate) fn execute(&self) -> Result<()> {
        let path = contrib::root_path()?;
        env::set_current_dir(&path)?;

        Ok(())
    }
}

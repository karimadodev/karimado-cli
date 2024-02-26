use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub(crate) struct InstallCommand {}

impl InstallCommand {
    pub(crate) fn execute(&self) -> Result<()> {
        Ok(())
    }
}

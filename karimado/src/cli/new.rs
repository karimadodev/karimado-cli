use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub(crate) struct NewCommand {}

impl NewCommand {
    pub(crate) fn execute(&self) -> Result<()> {
        Ok(())
    }
}

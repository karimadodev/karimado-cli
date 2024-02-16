use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct NewCommand {}

impl NewCommand {
    pub fn execute(_cmd: &Self) -> Result<()> {
        Ok(())
    }
}

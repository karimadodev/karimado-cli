use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub(crate) struct BuildCommand {
    /// Watch input files
    #[arg(long)]
    watch: bool,
}

impl BuildCommand {
    pub(crate) fn execute(&self) -> Result<()> {
        Ok(())
    }
}

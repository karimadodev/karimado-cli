use anyhow::Result;
use clap::Args;

use crate::{config, contrib};

#[derive(Args)]
pub(crate) struct RunCommand {}

impl RunCommand {
    pub(crate) fn execute(&self) -> Result<()> {
        let root_path = contrib::cli::root_path()?;
        let config_file_path = contrib::cli::config_file_path(&root_path);
        let config = config::from_config_file(&config_file_path)?;

        eprintln!("{:?}", config.tasks.taskfile);
        Ok(())
    }
}

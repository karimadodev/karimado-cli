use anyhow::Result;
use clap::Args;

use crate::{config, contrib, core::task};

#[derive(Args)]
pub(crate) struct RunCommand {}

impl RunCommand {
    pub(crate) fn execute(&self) -> Result<()> {
        let root_path = contrib::cli::root_path()?;
        let config_file_path = contrib::cli::config_file_path(&root_path);
        let config = config::from_config_file(&config_file_path)?;

        eprintln!("{:?}", config.tasks.taskfile);

        let taskfile_path = root_path.join(config.tasks.taskfile);
        let taskfile = task::taskfile::from_taskfile(&taskfile_path);
        eprintln!("{:?}", taskfile);
        Ok(())
    }
}

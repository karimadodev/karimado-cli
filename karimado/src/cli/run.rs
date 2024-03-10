use anyhow::Result;
use clap::Args;

use crate::{config, contrib, core::task};

#[derive(Args)]
pub(crate) struct RunCommand {
    /// Lists tasks with description of current taskfile
    #[arg(long, short)]
    list: bool,

    /// Executes tasks provided on command line in parallel
    #[arg(long, short)]
    parallel: bool,

    /// Task Name
    #[arg(default_value = "default")]
    task: Vec<String>,
}

impl RunCommand {
    pub(crate) fn execute(&self) -> Result<()> {
        let root_path = contrib::cli::root_path()?;
        let config_file_path = contrib::cli::config_file_path(&root_path);
        let config = config::from_config_file(&config_file_path)?;

        let taskfile = config.tasks.taskfile;
        let taskfile_path = root_path.join(taskfile);
        let taskmgr = task::TaskMgr::builder()
            .taskfile(&taskfile_path)
            .workdir(&root_path)
            .build()?;

        if self.list {
            taskmgr.list()?;
        } else if self.parallel {
            taskmgr.parallel_execute()?;
        } else {
            taskmgr.execute()?;
        }

        Ok(())
    }
}

use anyhow::Result;
use clap::Args;

use crate::{config, contrib};
use karimado_tasks;

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
        let taskmgr = karimado_tasks::TaskMgr::builder()
            .current_dir(&root_path)
            .taskfile(&taskfile)
            .build()?;

        if self.list || self.task.is_empty() {
            taskmgr.list();
            return Ok(());
        }

        let result = if self.parallel {
            taskmgr.parallel_execute(&self.task)
        } else {
            taskmgr.execute(&self.task)
        };
        if let Err(err) = result {
            if let karimado_tasks::Error::TaskNotFound(_) = err {
                taskmgr.list()
            }
            anyhow::bail!(err)
        } else {
            Ok(())
        }
    }
}

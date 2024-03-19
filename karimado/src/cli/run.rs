use anyhow::Result;
use clap::Args;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

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

    /// Task Args
    #[arg(raw = true)]
    args: Vec<String>,
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
            .cli_args(&self.args)
            .build()?;

        if self.list || self.task.is_empty() {
            taskmgr.list();
            return Ok(());
        }

        let terminated = Arc::new(AtomicBool::new(false));
        let watched_terminated = Arc::clone(&terminated);
        let watched_terminated_value = move || {
            if watched_terminated.load(Ordering::SeqCst) {
                Some("received Ctrl-C signal".to_string())
            } else {
                None
            }
        };
        ctrlc::set_handler(move || terminated.store(true, Ordering::SeqCst))
            .expect("failed to set Ctrl-C handler");

        let result = if self.parallel {
            taskmgr.parallel_execute(&self.task, watched_terminated_value)
        } else {
            taskmgr.execute(&self.task, watched_terminated_value)
        };

        if let Err(err) = result {
            if let karimado_tasks::Error::TaskNotFound(_) = err {
                let errmsg = format!("{}, use the `--list` flag to see all available tasks", err);
                anyhow::bail!(errmsg);
            } else {
                anyhow::bail!(err)
            }
        } else {
            Ok(())
        }
    }
}

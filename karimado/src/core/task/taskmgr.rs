use anyhow::Result;
use std::path::{Path, PathBuf};

use super::task::Task;

#[derive(Default)]
pub(crate) struct TaskMgr {
    tasks: Vec<Task>,
}

#[derive(Default)]
pub(crate) struct TaskMgrBuilder {
    taskfile: PathBuf,
    workdir: PathBuf,
}

impl TaskMgrBuilder {
    pub(crate) fn new() -> Self {
        Default::default()
    }

    pub(crate) fn taskfile(mut self, taskfile: &Path) -> Self {
        self.taskfile = taskfile.to_path_buf();
        self
    }

    pub(crate) fn workdir(mut self, workdir: &Path) -> Self {
        self.workdir = workdir.to_path_buf();
        self
    }

    pub(crate) fn build(self) -> Result<TaskMgr> {
        Ok(Default::default())
    }
}

impl TaskMgr {
    pub(crate) fn builder() -> TaskMgrBuilder {
        TaskMgrBuilder::new()
    }

    pub(crate) fn list(&self) -> Result<()> {
        for _task in &self.tasks {}
        Ok(())
    }

    pub(crate) fn parallel_execute(&self) -> Result<()> {
        eprintln!("Executes tasks provided on command line in parallel");
        Ok(())
    }

    pub(crate) fn execute(&self) -> Result<()> {
        eprintln!("Executes tasks provided on command line");
        Ok(())
    }
}

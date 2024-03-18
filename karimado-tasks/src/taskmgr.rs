mod build;
mod execute;
mod list;
mod parallel;

use std::path::{Path, PathBuf};

use crate::{error::*, Task};

pub struct TaskMgr {
    tasks: Vec<Task>,
}

#[derive(Default)]
pub struct TaskMgrBuilder {
    current_dir: PathBuf,
    taskfile: String,
    cli_args: Vec<String>,
}

impl TaskMgrBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn current_dir(mut self, current_dir: &Path) -> Self {
        self.current_dir = current_dir.to_path_buf();
        self
    }

    pub fn taskfile(mut self, taskfile: &str) -> Self {
        self.taskfile = taskfile.to_string();
        self
    }

    pub fn cli_args(mut self, args: &[String]) -> Self {
        self.cli_args = args.to_vec();
        self
    }

    pub fn build(self) -> Result<TaskMgr> {
        let tasks = build::build(&self.current_dir, &self.taskfile, &self.cli_args)?;
        Ok(TaskMgr { tasks })
    }
}

impl TaskMgr {
    pub fn builder() -> TaskMgrBuilder {
        TaskMgrBuilder::new()
    }

    pub fn list(&self) {
        list::list(&self.tasks)
    }

    pub fn parallel_execute<F: Fn() -> Option<String> + Send + 'static>(
        &self,
        task_names: &[String],
        watched: F,
    ) -> Result<()> {
        let tasks = self.lookup_tasks(task_names)?;
        parallel::execute(&tasks, watched)
    }

    pub fn execute(&self, task_names: &[String]) -> Result<()> {
        let tasks = self.lookup_tasks(task_names)?;
        execute::execute(&tasks)
    }

    fn lookup_tasks(&self, task_names: &[String]) -> Result<Vec<Task>> {
        task_names
            .iter()
            .map(|task_name| -> Result<Task> {
                let task_name = task_name.to_string();
                let task = self.tasks.iter().find(|task| task.name == task_name);
                task.ok_or_else(|| Error::TaskNotFound(task_name)).cloned()
            })
            .collect()
    }
}

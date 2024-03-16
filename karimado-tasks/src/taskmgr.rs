mod execute;
mod list;
mod parallel;

use crate::{error::*, Task};

pub struct TaskMgr {
    pub(super) tasks: Vec<Task>,
}

impl TaskMgr {
    pub fn list(&self) {
        list::list(&self.tasks)
    }

    pub fn parallel_execute(&self, task_names: &[String]) -> Result<()> {
        let tasks = self.lookup_tasks(task_names)?;
        parallel::execute(&tasks)?;
        Ok(())
    }

    pub fn execute(&self, task_names: &[String]) -> Result<()> {
        let tasks = self.lookup_tasks(task_names)?;
        execute::execute(&tasks)?;
        Ok(())
    }

    fn lookup_tasks(&self, task_names: &[String]) -> Result<Vec<Task>> {
        let mut tasks: Vec<Task> = vec![];
        for task_name in task_names {
            let task = self.tasks.iter().find(|task| task.name == *task_name);
            if let Some(task) = task {
                tasks.push(task.clone());
            } else {
                return Err(Error::TaskNotFound(task_name.to_string()));
            }
        }
        Ok(tasks)
    }
}

use anyhow::Result;
use colored::Colorize;
use std::path::{Path, PathBuf};

use super::{command::Command, task::Task, taskfile};

pub(crate) struct TaskMgr {
    tasks: Vec<Task>,
}

#[derive(Default)]
pub(crate) struct TaskMgrBuilder {
    taskfile: String,
    workdir: PathBuf,
}

#[derive(Default)]
struct TaskMgrBuilderBuildingContext {
    tasks: Vec<Task>,
    tasks_namespace: String,
    taskfile_dir: PathBuf,
}

impl TaskMgrBuilder {
    pub(crate) fn new() -> Self {
        Default::default()
    }

    pub(crate) fn taskfile(mut self, taskfile: &str) -> Self {
        self.taskfile = taskfile.to_string();
        self
    }

    pub(crate) fn workdir(mut self, workdir: &Path) -> Self {
        self.workdir = workdir.to_path_buf();
        self
    }

    pub(crate) fn build(self) -> Result<TaskMgr> {
        let mut ctx: TaskMgrBuilderBuildingContext = Default::default();
        let taskfile_path = self.workdir.join(&self.taskfile);

        log::debug!("parsing taskfile {}", taskfile_path.display());
        let taskfile = taskfile::from_taskfile(&taskfile_path)?;

        ctx.tasks_namespace = String::new();
        ctx.taskfile_dir = taskfile_path
            .parent()
            .expect("cannot resolve taskfile_dir")
            .to_path_buf();
        self.build_taskfile(&mut ctx, &taskfile)?;

        let mut tasks = ctx.tasks;
        tasks.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(TaskMgr { tasks })
    }

    fn build_taskfile(
        &self,
        ctx: &mut TaskMgrBuilderBuildingContext,
        taskfile: &taskfile::Taskfile,
    ) -> Result<()> {
        self.build_taskfile_includes(ctx, &taskfile.includes)?;
        self.build_taskfile_tasks(ctx, &taskfile.tasks)?;
        Ok(())
    }

    fn build_taskfile_includes(
        &self,
        ctx: &mut TaskMgrBuilderBuildingContext,
        includes: &[taskfile::Include],
    ) -> Result<()> {
        for include in includes {
            self.build_taskfile_include(ctx, include)?;
        }
        Ok(())
    }

    fn build_taskfile_include(
        &self,
        ctx: &mut TaskMgrBuilderBuildingContext,
        include: &taskfile::Include,
    ) -> Result<()> {
        let taskfile_path = ctx.taskfile_dir.join(&include.taskfile);
        if !taskfile_path.exists() {
            if include.optional {
                return Ok(());
            }
            anyhow::bail!(
                "taskfile `{}` does not exists under {}",
                include.taskfile,
                ctx.taskfile_dir.display()
            );
        }

        log::debug!("parsing taskfile {}", taskfile_path.display());
        let taskfile = taskfile::from_taskfile(&taskfile_path)?;

        let old_tasks_namespace = ctx.tasks_namespace.clone();
        let old_taskfile_dir = ctx.taskfile_dir.clone();

        let new_tasks_namespace = if !ctx.tasks_namespace.is_empty() {
            format!("{}:{}", ctx.tasks_namespace, include.name)
        } else {
            include.name.to_string()
        };
        let new_taskfile_dir = taskfile_path
            .parent()
            .expect("cannot resolve taskfile_dir")
            .to_path_buf();

        ctx.tasks_namespace = new_tasks_namespace;
        ctx.taskfile_dir = new_taskfile_dir;
        self.build_taskfile(ctx, &taskfile)?;
        ctx.taskfile_dir = old_taskfile_dir;
        ctx.tasks_namespace = old_tasks_namespace;

        Ok(())
    }

    fn build_taskfile_tasks(
        &self,
        ctx: &mut TaskMgrBuilderBuildingContext,
        tasks: &[taskfile::Task],
    ) -> Result<()> {
        for task in tasks {
            self.build_taskfile_task(ctx, task)?;
        }
        Ok(())
    }

    fn build_taskfile_task(
        &self,
        ctx: &mut TaskMgrBuilderBuildingContext,
        task: &taskfile::Task,
    ) -> Result<()> {
        let task_name = if !ctx.tasks_namespace.is_empty() {
            format!("{}:{}", ctx.tasks_namespace, task.name)
        } else {
            task.name.clone()
        };

        ctx.tasks.push(Task {
            name: task_name,
            desc: task.desc.clone(),
            command: task.command.clone(),
            current_dir: self.workdir.clone(),
        });

        Ok(())
    }
}

impl TaskMgr {
    pub(crate) fn builder() -> TaskMgrBuilder {
        TaskMgrBuilder::new()
    }

    pub(crate) fn list(&self) -> Result<()> {
        let task_name = |task: &Task| format!("{}", task.name.green());
        let maxwidth = self
            .tasks
            .iter()
            .map(|task| task_name(task).len())
            .max()
            .unwrap_or(0);

        log::info!("Available tasks for this project:");
        for task in &self.tasks {
            let task_name = format!("{:<width$}", task_name(task), width = maxwidth);
            let task_desc = format!("# {}", task.desc);
            log::info!("{} {} {}", "*".yellow(), task_name, task_desc);
        }

        Ok(())
    }

    pub(crate) fn parallel_execute(&self, task_names: &[String]) -> Result<()> {
        let _ = self.lookup_tasks(task_names)?;
        Ok(())
    }

    pub(crate) fn execute(&self, task_names: &[String]) -> Result<()> {
        let tasks = self.lookup_tasks(task_names)?;
        for task in tasks {
            log::info!("$ {}", task.command);
            let mut cmd = Command::new(&task.command)
                .current_dir(&task.current_dir)
                .spawn()
                .expect("failed to execute command");
            match cmd.wait()?.code() {
                Some(0) => {
                    log::info!("");
                }
                Some(code) => {
                    anyhow::bail!("failed to run task `{}`: exit status {}", task.name, code)
                }
                None => {
                    anyhow::bail!("failed to run task `{}`: exit status unknown", task.name)
                }
            }
        }
        Ok(())
    }

    fn lookup_tasks(&self, task_names: &[String]) -> Result<Vec<Task>> {
        let mut tasks: Vec<Task> = vec![];
        for task_name in task_names {
            let task = self.tasks.iter().find(|task| task.name == *task_name);
            if let Some(task) = task {
                tasks.push(task.clone());
            } else {
                anyhow::bail!("task `{}` does not exists", task_name);
            }
        }
        Ok(tasks)
    }
}

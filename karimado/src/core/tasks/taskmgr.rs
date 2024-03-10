use anyhow::Result;
use std::path::{Path, PathBuf};

use super::{task::Task, taskfile};

#[derive(Default)]
pub(crate) struct TaskMgr {
    tasks: Vec<Task>,
}

#[derive(Default)]
pub(crate) struct TaskMgrBuilder {
    taskfile: PathBuf,
    workdir: PathBuf,
}

struct TaskMgrBuilderBuildingContext {
    taskfile_dir: PathBuf,
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
        let ctx = TaskMgrBuilderBuildingContext {
            taskfile_dir: self.taskfile.parent().unwrap().to_path_buf(),
        };
        let taskfile = taskfile::from_taskfile(&self.taskfile)?;
        self.build_taskfile(&ctx, &taskfile)?;

        Ok(Default::default())
    }

    fn build_taskfile(
        &self,
        ctx: &TaskMgrBuilderBuildingContext,
        taskfile: &taskfile::Taskfile,
    ) -> Result<()> {
        self.build_taskfile_includes(ctx, &taskfile.includes)?;
        self.build_taskfile_tasks(ctx, &taskfile.tasks)?;
        Ok(())
    }

    fn build_taskfile_includes(
        &self,
        ctx: &TaskMgrBuilderBuildingContext,
        includes: &[taskfile::Include],
    ) -> Result<()> {
        for i in includes {
            self.build_taskfile_include(ctx, i)?;
        }
        Ok(())
    }

    fn build_taskfile_include(
        &self,
        _ctx: &TaskMgrBuilderBuildingContext,
        _include: &taskfile::Include,
    ) -> Result<()> {
        Ok(())
    }

    fn build_taskfile_tasks(
        &self,
        ctx: &TaskMgrBuilderBuildingContext,
        tasks: &[taskfile::Task],
    ) -> Result<()> {
        for t in tasks {
            self.build_taskfile_task(ctx, t)?;
        }
        Ok(())
    }

    fn build_taskfile_task(
        &self,
        _ctx: &TaskMgrBuilderBuildingContext,
        _task: &taskfile::Task,
    ) -> Result<()> {
        Ok(())
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

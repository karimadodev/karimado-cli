#[cfg(test)]
#[path = "build_test.rs"]
mod tests;

use handlebars::Handlebars;
use serde_json::json;
use std::path::{Path, PathBuf};

use crate::{error::*, taskfile, Task};

#[derive(Default)]
struct BuildingContext {
    // constants
    working_dir: PathBuf,
    cli_args: String,

    // realtime
    tasks_namespace: Vec<String>,
    tasks_cwd: PathBuf,
    taskfile_dir: PathBuf,

    // result
    tasks: Vec<Task>,
}

pub(crate) fn build(current_dir: &Path, taskfile: &str, cli_args: &[String]) -> Result<Vec<Task>> {
    let mut ctx = BuildingContext {
        working_dir: current_dir.to_path_buf(),
        cli_args: cli_args.join(" "),
        ..Default::default()
    };
    let taskfile_path = current_dir.join(taskfile);

    log::debug!("parsing taskfile {}", taskfile_path.display());
    let taskfile = taskfile::from_taskfile(&taskfile_path)?;

    ctx.tasks_namespace = vec![];
    ctx.tasks_cwd = current_dir.to_path_buf();
    ctx.taskfile_dir = taskfile_path
        .parent()
        .expect("failed to resolve taskfile dir")
        .to_path_buf();
    build_taskfile(&taskfile, &mut ctx)?;

    let mut tasks = ctx.tasks;
    tasks.sort_by_key(|task| task.name.clone());
    Ok(tasks)
}

fn build_taskfile(taskfile: &taskfile::Taskfile, ctx: &mut BuildingContext) -> Result<()> {
    build_taskfile_includes(&taskfile.includes, ctx)?;
    build_taskfile_tasks(&taskfile.tasks, ctx)?;
    Ok(())
}

fn build_taskfile_includes(
    includes: &[taskfile::Include],
    ctx: &mut BuildingContext,
) -> Result<()> {
    for include in includes {
        build_taskfile_include(include, ctx)?;
    }
    Ok(())
}

fn build_taskfile_include(include: &taskfile::Include, ctx: &mut BuildingContext) -> Result<()> {
    let taskfile_path = ctx.taskfile_dir.join(&include.taskfile);
    if !taskfile_path.exists() {
        if include.optional {
            return Ok(());
        }

        let taskfile = &include.taskfile;
        let dir = ctx.taskfile_dir.display();
        let e = format!("taskfile `{}` does not exists under {}", taskfile, dir);
        Err(TaskFileParseErrorKind::ParseIncludeError(e))?;
    }

    log::debug!("parsing taskfile {}", taskfile_path.display());
    let taskfile = taskfile::from_taskfile(&taskfile_path)?;

    let old_tasks_namespace = ctx.tasks_namespace.clone();
    let old_tasks_cwd = ctx.tasks_cwd.clone();
    let old_taskfile_dir = ctx.taskfile_dir.clone();

    let new_tasks_namespace = {
        let mut v = ctx.tasks_namespace.clone();
        v.push(include.name.clone());
        v
    };
    let new_tasks_cwd = if let Some(wd) = &include.working_dir {
        ctx.working_dir.join(wd)
    } else {
        ctx.tasks_cwd.clone()
    };
    let new_taskfile_dir = taskfile_path
        .parent()
        .expect("failed to resolve taskfile dir")
        .to_path_buf();

    ctx.tasks_namespace = new_tasks_namespace;
    ctx.tasks_cwd = new_tasks_cwd;
    ctx.taskfile_dir = new_taskfile_dir;
    build_taskfile(&taskfile, ctx)?;
    ctx.tasks_namespace = old_tasks_namespace;
    ctx.tasks_cwd = old_tasks_cwd;
    ctx.taskfile_dir = old_taskfile_dir;

    Ok(())
}

fn build_taskfile_tasks(tasks: &[taskfile::Task], ctx: &mut BuildingContext) -> Result<()> {
    for task in tasks {
        build_taskfile_task(task, ctx)?;
    }
    Ok(())
}

fn build_taskfile_task(task: &taskfile::Task, ctx: &mut BuildingContext) -> Result<()> {
    let name = {
        let mut v = ctx.tasks_namespace.clone();
        v.push(task.name.clone());
        v.join(":")
    };
    let command = build_taskfile_task_command(task, ctx)?;
    let description = task.description.clone();
    let current_dir = build_taskfile_task_current_dir(task, ctx)?;

    ctx.tasks.push(Task {
        name,
        command,
        description,
        current_dir,
    });

    Ok(())
}

fn build_taskfile_task_command(task: &taskfile::Task, ctx: &mut BuildingContext) -> Result<String> {
    let renderer = Handlebars::new();
    let vars = &json!({
        "CLI_ARGS": ctx.cli_args
    });
    let command = renderer
        .render_template(&task.command, &vars)
        .map_err(TaskFileParseErrorKind::ParseTaskCommandError)?;
    Ok(command)
}

fn build_taskfile_task_current_dir(
    task: &taskfile::Task,
    ctx: &mut BuildingContext,
) -> Result<PathBuf> {
    let cwd = if let Some(wd) = &task.working_dir {
        ctx.working_dir.join(wd)
    } else {
        ctx.tasks_cwd.clone()
    };
    Ok(cwd)
}

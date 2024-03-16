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

    Ok(ctx.tasks)
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
        Err(TaskFileParseFailedKind::ParseIncludeFailed(e))?;
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
    let new_tasks_cwd = ctx.working_dir.join(&include.working_dir);
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
    let command = build_taskfile_task_command(&task.command, ctx)?;
    let description = task.description.clone();
    let current_dir = ctx.tasks_cwd.clone();

    ctx.tasks.push(Task {
        name,
        command,
        description,
        current_dir,
    });

    Ok(())
}

fn build_taskfile_task_command(command: &str, ctx: &mut BuildingContext) -> Result<String> {
    let renderer = Handlebars::new();
    let vars = &json!({
        "CLI_ARGS": ctx.cli_args
    });
    let command = renderer
        .render_template(command, &vars)
        .map_err(TaskFileParseFailedKind::ParseTaskCommandFailed)?;
    Ok(command)
}

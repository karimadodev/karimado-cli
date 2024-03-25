#[cfg(test)]
#[path = "build_test.rs"]
mod tests;

use serde_json::{json, Map as JsonMap, Value as JsonValue};
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
    let vars = build_taskfile_task_command_vars(task, ctx);
    let h1 = Box::new(hbs_helper_getvar_or_default(vars.clone()));

    let mut renderer = handlebars::Handlebars::new();
    renderer.register_helper("getvar_or_default", h1);

    let command = renderer
        .render_template(&task.command, &vars)
        .map_err(TaskFileParseErrorKind::ParseTaskCommandError)?;
    Ok(command)
}

fn build_taskfile_task_command_vars(
    _task: &taskfile::Task,
    ctx: &mut BuildingContext,
) -> JsonMap<String, JsonValue> {
    let mut map = JsonMap::new();
    map.insert("CLI_ARGS".to_string(), json!(ctx.cli_args));
    map
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

fn hbs_helper_getvar_or_default(
    vars: JsonMap<String, JsonValue>,
) -> impl for<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i> Fn(
    &'a handlebars::Helper<'b>,
    &'c handlebars::Handlebars<'d>,
    &'e handlebars::Context,
    &'f mut handlebars::RenderContext<'g, 'h>,
    &'i mut (dyn handlebars::Output + 'i),
) -> std::result::Result<(), handlebars::RenderError> {
    move |h: &handlebars::Helper,
          _: &handlebars::Handlebars,
          _c: &handlebars::Context,
          _rc: &mut handlebars::RenderContext,
          out: &mut dyn handlebars::Output|
          -> handlebars::HelperResult {
        let name = h.param(0).and_then(|v| v.value().as_str()).ok_or(
            handlebars::RenderErrorReason::ParamTypeMismatchForName(
                "`getvar_or_default`",
                "`var_name`".to_string(),
                "String".to_string(),
            ),
        )?;
        let default = h.param(1).and_then(|v| v.value().as_str()).ok_or(
            handlebars::RenderErrorReason::ParamTypeMismatchForName(
                "`getvar_or_default`",
                "`default`".to_string(),
                "String".to_string(),
            ),
        )?;

        let value = &vars.get(name);
        if value.is_none() {
            let e = format!("var `{}` is undefined", name);
            Err(handlebars::RenderErrorReason::Other(e))?;
        }

        match value.expect("Option::unwrap()") {
            JsonValue::String(str) => {
                if str.is_empty() {
                    out.write(default)?;
                } else {
                    out.write(str)?;
                }
            }
            _ => unreachable!(),
        };

        Ok(())
    }
}

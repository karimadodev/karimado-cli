use anyhow::Result;
use colored::Colorize;

use super::super::task::Task;

pub(crate) fn list(tasks: &[Task]) -> Result<()> {
    let task_name = |task: &Task| task.name.green();
    let maxwidth = tasks
        .iter()
        .map(|task| task_name(task).len())
        .max()
        .unwrap_or(0);

    log::info!("Available tasks for this project:");
    for task in tasks {
        let task_icon = "*".yellow();
        let task_name = format!("{:<width$}", task_name(task), width = maxwidth);
        let task_desc = format!("# {}", task.description.as_ref().unwrap_or(&task.command));
        log::info!("{} {} {}", task_icon, task_name, task_desc);
    }
    Ok(())
}

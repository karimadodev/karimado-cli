#[cfg(test)]
#[path = "list_test.rs"]
mod tests;

use colored::Colorize;

use crate::Task;

pub(crate) fn list(tasks: &[Task]) {
    let colored_task_name = |name: &str| name.green();
    let maxwidth = tasks
        .iter()
        .map(|task| colored_task_name(&task.name).len())
        .max()
        .unwrap_or(0);

    println!("Available tasks for this project:");
    for task in tasks {
        let task_icon = "*".yellow();
        let task_name = format!("{:<w$}", colored_task_name(&task.name), w = maxwidth);
        let task_desc = format!("# {}", task.description.as_ref().unwrap_or(&task.command));
        println!("{} {} {}", task_icon, task_name, task_desc);
    }
}

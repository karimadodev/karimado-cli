use anyhow::Result;
use colored::Colorize;

use crate::{shell, task::Task};

pub(crate) fn execute(tasks: &[Task]) -> Result<()> {
    for task in tasks {
        log::info!("{}", format!("$ {}", task.command).green());
        let mut child = shell::command(&task.command)
            .current_dir(&task.current_dir)
            .spawn()
            .expect("failed to execute command");

        let status = child.wait().expect("command wasn't running");
        let code = status.code();
        match code {
            Some(0) => log::info!(""),
            Some(code) => anyhow::bail!("failed to run task `{}`: exit code {}", task.name, code),
            None => anyhow::bail!("failed to run task `{}`: terminated by signal", task.name),
        }
    }
    Ok(())
}

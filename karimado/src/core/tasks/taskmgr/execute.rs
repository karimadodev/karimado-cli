use anyhow::Result;

use super::super::{shell, task::Task};

pub(crate) fn execute(tasks: &[Task]) -> Result<()> {
    for task in tasks {
        log::info!("$ {}", task.command);
        let mut child = shell::command(&task.command)
            .current_dir(&task.current_dir)
            .spawn()
            .expect("failed to execute command");

        let status = child.wait().expect("command wasn't running");
        match status.code() {
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

use colored::Colorize;

use crate::{error::*, shell, Task};

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
            Some(0) => {
                log::info!("");
                continue;
            }
            Some(c) => {
                let e = format!("failed to run task `{}`, exited with code {}", task.name, c);
                return Err(Error::TaskRunFailed(e));
            }
            None => {
                let e = format!("failed to run task `{}`, terminated by signal", task.name);
                return Err(Error::TaskRunFailed(e));
            }
        }
    }
    Ok(())
}

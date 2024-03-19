#[cfg(test)]
#[path = "execute_test.rs"]
mod tests;

use colored::Colorize;
use std::process::ExitStatus;
use std::thread;
use std::time::Duration;

use crate::{error::*, shell, Task};

pub(crate) fn execute<F: Fn() -> Option<String> + Send + 'static>(
    tasks: &[Task],
    watched: F,
) -> Result<()> {
    for task in tasks {
        log::info!("{}", format!("-> {}", task.command).green());
        let mut child = shell::command(&task.command)
            .current_dir(&task.current_dir)
            .spawn()
            .expect("failed to execute command");

        loop {
            thread::sleep(Duration::from_millis(167));

            // terminated by user
            if let Some(errmsg) = watched() {
                child.kill().expect("failed to kill command");
                child.wait().expect("failed to wait command");
                return Err(Error::TaskRunFailed(errmsg));
            }

            // terminated by taskmgr
            let status = child.try_wait().expect("failed to try wait");
            if let Some(status) = status {
                // finished task: exit with Err if failed, otherwise goto next task
                log::info!("");
                handle_finished_task_exit_status(task, &status)?;
                break;
            } else {
                // unfinsihed task: waiting to be done
                continue;
            }
        }
    }
    Ok(())
}

fn handle_finished_task_exit_status(task: &Task, status: &ExitStatus) -> Result<()> {
    if status.success() {
        return Ok(());
    }

    let errmsg = if let Some(c) = status.code() {
        format!("failed to run task `{}`, exited with code {}", task.name, c)
    } else {
        format!("failed to run task `{}`, terminated by signal", task.name)
    };
    Err(Error::TaskRunFailed(errmsg))
}

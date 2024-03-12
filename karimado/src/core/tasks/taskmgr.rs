use anyhow::Result;
use colored::Colorize;
use std::io::BufRead;

use super::{command, task::Task};

pub(crate) struct TaskMgr {
    pub(super) tasks: Vec<Task>,
}

impl TaskMgr {
    pub(crate) fn list(&self) -> Result<()> {
        let task_name = |task: &Task| task.name.green();
        let maxwidth = self
            .tasks
            .iter()
            .map(|task| task_name(task).len())
            .max()
            .unwrap_or(0);

        log::info!("Available tasks for this project:");
        for task in &self.tasks {
            let task_name = format!("{:<width$}", task_name(task), width = maxwidth);
            let task_desc = format!("# {}", task.description.as_ref().unwrap_or(&task.command));
            log::info!("{} {} {}", "*".yellow(), task_name, task_desc);
        }

        Ok(())
    }

    pub(crate) fn parallel_execute(&self, task_names: &[String]) -> Result<()> {
        let task_name = |task: &Task| format!("{} |", task.name).green();
        let maxwidth = self
            .tasks
            .iter()
            .map(|task| task_name(task).len())
            .max()
            .unwrap_or(0);

        let tasks = self.lookup_tasks(task_names)?;
        let mut stdout_thrs: Vec<std::thread::JoinHandle<()>> = vec![];
        let mut stderr_thrs: Vec<std::thread::JoinHandle<()>> = vec![];
        let mut waiter_thrs: Vec<std::thread::JoinHandle<()>> = vec![];

        for task in tasks {
            log::info!(
                "{:>width$} {}",
                task_name(&task),
                format!("$ {}", task.command),
                width = maxwidth
            );
            let mut child = command::command(&task.command)
                .current_dir(&task.current_dir)
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()
                .expect("failed to execute command");

            let stdout_task_name = task_name(&task);
            let stdout_reader =
                std::io::BufReader::new(child.stdout.take().expect("failed to take stdout"));
            stdout_thrs.push(std::thread::spawn(move || {
                for line in stdout_reader.lines().map_while(Result::ok) {
                    log::info!("{:>width$} {}", stdout_task_name, line, width = maxwidth);
                }
            }));

            let stderr_task_name = task_name(&task);
            let stderr_reader =
                std::io::BufReader::new(child.stderr.take().expect("failed to take stderr"));
            stderr_thrs.push(std::thread::spawn(move || {
                for line in stderr_reader.lines().map_while(Result::ok) {
                    log::info!("{:>width$} {}", stderr_task_name, line, width = maxwidth);
                }
            }));

            waiter_thrs.push(std::thread::spawn(move || {
                let _status = child.wait().expect("command wasn't running");
            }));
        }

        stdout_thrs
            .into_iter()
            .for_each(move |thr| thr.join().expect("failed to join on the associated thread"));
        stderr_thrs
            .into_iter()
            .for_each(move |thr| thr.join().expect("failed to join on the associated thread"));
        waiter_thrs
            .into_iter()
            .for_each(move |thr| thr.join().expect("failed to join on the associated thread"));

        Ok(())
    }

    pub(crate) fn execute(&self, task_names: &[String]) -> Result<()> {
        let tasks = self.lookup_tasks(task_names)?;

        for task in tasks {
            log::info!("$ {}", task.command);
            let mut child = command::command(&task.command)
                .current_dir(&task.current_dir)
                .spawn()
                .expect("failed to execute command");

            match child.wait()?.code() {
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

    fn lookup_tasks(&self, task_names: &[String]) -> Result<Vec<Task>> {
        let mut tasks: Vec<Task> = vec![];
        for task_name in task_names {
            let task = self.tasks.iter().find(|task| task.name == *task_name);
            if let Some(task) = task {
                tasks.push(task.clone());
            } else {
                anyhow::bail!("task `{}` does not exists", task_name);
            }
        }
        Ok(tasks)
    }
}

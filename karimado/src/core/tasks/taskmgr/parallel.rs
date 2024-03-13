use anyhow::Result;
use colored::Colorize;
use std::{
    io::{BufRead, BufReader},
    process::Stdio,
    thread,
};

use super::super::{shell, task::Task};

pub(crate) fn execute(tasks: &[Task]) -> Result<()> {
    let task_name = |task: &Task| format!("{} |", task.name).purple();
    let maxwidth = tasks
        .iter()
        .map(|task| task_name(task).len())
        .max()
        .unwrap_or(0);

    let mut stdout_thrs: Vec<thread::JoinHandle<()>> = vec![];
    let mut stderr_thrs: Vec<thread::JoinHandle<()>> = vec![];
    let mut waiter_thrs: Vec<thread::JoinHandle<()>> = vec![];

    for task in tasks {
        let task_name = task_name(task);
        let line = format!("$ {}", task.command);
        log::info!("{:>width$} {}", task_name, line, width = maxwidth);
        let mut child = shell::command(&task.command)
            .current_dir(&task.current_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to execute command");

        let stdout_task_name = task_name.clone();
        let stdout_reader = BufReader::new(child.stdout.take().expect("failed to take stdout"));
        stdout_thrs.push(thread::spawn(move || {
            for line in stdout_reader.lines().map_while(Result::ok) {
                log::info!("{:>width$} {}", stdout_task_name, line, width = maxwidth);
            }
        }));

        let stderr_task_name = task_name.clone();
        let stderr_reader = BufReader::new(child.stderr.take().expect("failed to take stderr"));
        stderr_thrs.push(thread::spawn(move || {
            for line in stderr_reader.lines().map_while(Result::ok) {
                log::info!("{:>width$} {}", stderr_task_name, line, width = maxwidth);
            }
        }));

        let waiter_task_name = task_name.clone();
        waiter_thrs.push(thread::spawn(move || {
            let status = child.wait().expect("command wasn't running");
            let line = format!("# exit status: {}", status.code().unwrap_or(-1));
            log::debug!("{:>width$} {}", waiter_task_name, line, width = maxwidth);
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

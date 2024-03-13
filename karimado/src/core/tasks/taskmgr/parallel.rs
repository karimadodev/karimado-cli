use anyhow::Result;
use colored::Colorize;
use flurry::HashMap;
use shared_child::SharedChild;
use std::{
    io::{BufRead, BufReader},
    process::Stdio,
    sync::{mpsc, Arc, Mutex},
    thread,
};

use super::super::{shell, task::Task};

pub(crate) fn execute(tasks: &[Task]) -> Result<()> {
    let colored_task_name = |name: &str| format!("{} |", name).purple();
    let maxwidth = tasks
        .iter()
        .map(|task| colored_task_name(&task.name).len())
        .max()
        .unwrap_or(0);

    let mut stdout_thrs: Vec<thread::JoinHandle<()>> = vec![];
    let mut stderr_thrs: Vec<thread::JoinHandle<()>> = vec![];
    let mut waiter_thrs: Vec<thread::JoinHandle<()>> = vec![];

    let (tx, rx) = mpsc::channel::<(i32, usize, i32)>();
    let children: HashMap<usize, (String, Arc<SharedChild>)> = HashMap::with_capacity(tasks.len());

    // children: spawn all tasks
    for (task_id, task) in tasks.iter().enumerate() {
        let task_name = colored_task_name(&task.name);
        let line = format!("$ {}", task.command).green();
        log::info!("{:>w$} {}", task_name, line, w = maxwidth);

        // child: spawn
        let child = Arc::new(
            SharedChild::spawn(
                shell::command(&task.command)
                    .current_dir(&task.current_dir)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped()),
            )
            .expect("failed to execute command"),
        );

        // child: stdout reader
        let stdout_task_name = task_name.clone();
        let stdout_reader = BufReader::new(child.take_stdout().expect("failed to take stdout"));
        stdout_thrs.push(thread::spawn(move || {
            for line in stdout_reader.lines().map_while(Result::ok) {
                log::info!("{:>w$} {}", stdout_task_name, line, w = maxwidth);
            }
        }));

        // child: stderr reader
        let stderr_task_name = task_name.clone();
        let stderr_reader = BufReader::new(child.take_stderr().expect("failed to take stderr"));
        stderr_thrs.push(thread::spawn(move || {
            for line in stderr_reader.lines().map_while(Result::ok) {
                log::info!("{:>w$} {}", stderr_task_name, line, w = maxwidth);
            }
        }));

        // child: waiter
        let waiter_tx = tx.clone();
        let waiter_child = child.clone();
        waiter_thrs.push(thread::spawn(move || {
            let status = waiter_child.wait().expect("command wasn't running");
            let code = status.code().unwrap_or(-1);
            match code {
                0 => (),
                _ => _ = waiter_tx.send((1, task_id, code)),
            }
        }));

        // child: append to children
        children.pin().insert(task_id, (task.name.clone(), child));
    }

    // retval:
    let retval = Arc::new(Mutex::new(String::new()));

    // watcher: collect all tasks's result -> retval
    let watcher_retval = Arc::clone(&retval);
    let watcher_thr = thread::spawn(move || {
        let (reason, id, code) = rx.recv().expect("failed to recv");
        // reason:
        //   0: all tasks succeed
        //   1: one of the tasks had failed
        match reason {
            0 => {}
            1 => {
                for (task_id, (task_name, child)) in &children.pin() {
                    // force kill unfinished tasks
                    if *task_id != id {
                        let watcher_task_name = colored_task_name(task_name);
                        let line = format!("terminating `{}`", task_name);
                        log::info!("{:>w$} {}", watcher_task_name, line.yellow(), w = maxwidth);
                        child.kill().expect("failed to kill command");
                        continue;
                    }

                    // failure task
                    let watcher_task_name = colored_task_name(task_name);
                    let line = format!("failed to run task `{}`: exit code {}", task_name, code);
                    log::info!("{:>w$} {}", watcher_task_name, line.red(), w = maxwidth);
                    *watcher_retval.lock().expect("failed to lock data") = line.to_string();
                }
            }
            _ => unreachable!(),
        }
    });

    // children: wait for all tasks to be finished or to be killed
    stdout_thrs
        .into_iter()
        .for_each(move |thr| thr.join().expect("failed to join on the associated thread"));
    stderr_thrs
        .into_iter()
        .for_each(move |thr| thr.join().expect("failed to join on the associated thread"));
    waiter_thrs
        .into_iter()
        .for_each(move |thr| thr.join().expect("failed to join on the associated thread"));

    // watcher:
    _ = tx.send((0, 0, 0));
    watcher_thr
        .join()
        .expect("failed to join on the associated thread");

    // retval:
    let retval = retval.lock().expect("failed to lock data");
    if retval.is_empty() {
        Ok(())
    } else {
        anyhow::bail!(retval.clone());
    }
}

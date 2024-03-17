#[cfg(test)]
#[path = "parallel_test.rs"]
mod tests;

use colored::Colorize;
use flurry::HashMap;
use shared_child::SharedChild;
use std::{
    io::{BufRead, BufReader},
    process::Stdio,
    sync::{mpsc, Arc, Mutex},
    thread,
};

use crate::{error::*, shell, Task};

const TASKS_SUCCESS: i32 = 0; // all tasks succeed
const TASKS_FAILURE: i32 = 1; // one of the tasks had failed
const TASKS_CTRL_C_: i32 = 2; // received Ctrl-C signal

pub(crate) fn execute(tasks: &[Task]) -> Result<()> {
    let colored_task_name = |name: &str| format!(" {} |", name).purple();
    let maxwidth = tasks
        .iter()
        .map(|task| colored_task_name(&task.name).len())
        .max()
        .unwrap_or(0);

    let mut stdout_thrs: Vec<thread::JoinHandle<()>> = vec![];
    let mut stderr_thrs: Vec<thread::JoinHandle<()>> = vec![];
    let mut waiter_thrs: Vec<thread::JoinHandle<()>> = vec![];

    let (tx, rx) = mpsc::channel::<i32>();
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
            for line in stdout_reader.lines() {
                if let Ok(line) = line {
                    log::info!("{:>w$} {}", stdout_task_name, line, w = maxwidth);
                } else if let Err(line) = line {
                    log::warn!("{:>w$} {}", stdout_task_name, line, w = maxwidth);
                }
            }
        }));

        // child: stderr reader
        let stderr_task_name = task_name.clone();
        let stderr_reader = BufReader::new(child.take_stderr().expect("failed to take stderr"));
        stderr_thrs.push(thread::spawn(move || {
            for line in stderr_reader.lines() {
                if let Ok(line) = line {
                    log::info!("{:>w$} {}", stderr_task_name, line, w = maxwidth);
                } else if let Err(line) = line {
                    log::warn!("{:>w$} {}", stderr_task_name, line, w = maxwidth);
                }
            }
        }));

        // child: waiter
        let waiter_tx = tx.clone();
        let waiter_child = child.clone();
        let waiter_task_name = task_name.clone();
        waiter_thrs.push(thread::spawn(move || {
            let status = waiter_child.wait().expect("command wasn't running");
            let code = status.code();
            match code {
                Some(0) => {
                    let line = "task finished".to_string();
                    log::info!("{:>w$} {}", waiter_task_name, line.blue(), w = maxwidth);
                }
                Some(c) => {
                    let line = format!("task exited with code {}", c);
                    log::info!("{:>w$} {}", waiter_task_name, line.red(), w = maxwidth);
                    _ = waiter_tx.send(TASKS_FAILURE)
                }
                None => {
                    let line = "task terminated".to_string();
                    log::info!("{:>w$} {}", waiter_task_name, line.yellow(), w = maxwidth);
                    _ = waiter_tx.send(TASKS_FAILURE)
                }
            }
        }));

        // child: append to children
        children.pin().insert(task_id, (task.name.clone(), child));
    }

    // retval: the value used to store execution result
    let retval = Arc::new(Mutex::new(String::new()));
    let retval_init = |retval: &Arc<Mutex<String>>, errmsg: &str| {
        let mut retval = retval.lock().expect("failed to lock data");
        if (*retval).is_empty() {
            *retval = errmsg.to_string();
        };
    };

    // watcher: collect execution result into retval
    let watcher_retval = Arc::clone(&retval);
    let watcher_reap = move |reason| {
        if reason == TASKS_CTRL_C_ {
            let err = "received Ctrl-C signal";
            retval_init(&watcher_retval, err);
        }

        for (_task_id, (task_name, child)) in &children.pin() {
            // unfinished tasks: force kill
            let status = child.try_wait().expect("failed to try wait");
            if status.is_none() {
                child.kill().expect("failed to kill command");
                continue;
            }

            // finished tasks: collect execution result
            let code = status.expect("Option::unwrap()").code();
            match code {
                Some(0) => {}
                Some(c) => {
                    let e = format!("failed to run task `{}`, exited with code {}", task_name, c);
                    retval_init(&watcher_retval, &e);
                }
                None => {
                    let e = format!("failed to run task `{}`, terminated by signal", task_name);
                    retval_init(&watcher_retval, &e);
                }
            }
        }
    };
    let watcher_thr = thread::spawn(move || {
        let reason = rx.recv().expect("failed to recv"); // peek the first value
        match reason {
            TASKS_SUCCESS => {}
            TASKS_FAILURE => watcher_reap(reason),
            TASKS_CTRL_C_ => watcher_reap(reason),
            _ => unreachable!(),
        }
    });

    // ctrlc:
    let ctrlc_tx = tx.clone();
    ctrlc::set_handler(move || _ = ctrlc_tx.send(TASKS_CTRL_C_))
        .expect("failed to set Ctrl-C handler");

    // children: wait for all tasks to be done
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
    _ = tx.send(TASKS_SUCCESS);
    watcher_thr
        .join()
        .expect("failed to join on the associated thread");

    // retval:
    let retval = retval.lock().expect("failed to lock data");
    if retval.is_empty() {
        Ok(())
    } else {
        Err(Error::TaskRunFailed(retval.to_string()))
    }
}

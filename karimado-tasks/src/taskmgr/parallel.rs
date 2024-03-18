#[cfg(test)]
#[path = "parallel_test.rs"]
mod tests;

use colored::Colorize;
use flurry::HashMap;
use shared_child::SharedChild;
use std::{
    io::{BufRead, BufReader},
    process::Stdio,
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use crate::{error::*, shell, Task};

const TASKS_WAITING: i32 = 0; // waiting to be done
const TASKS_SUCCESS: i32 = 1; // done: all tasks succeed
const TASKS_FAILURE: i32 = 2; // done: one of the tasks had failed

pub(crate) fn execute<F: Fn() -> Option<String> + Send + 'static>(
    tasks: &[Task],
    watched: F,
) -> Result<()> {
    let colored_task_name = |name: &str| format!(" {} |", name).purple();
    let maxwidth = tasks
        .iter()
        .map(|task| colored_task_name(&task.name).len())
        .max()
        .unwrap_or(0);

    let mut stdout_thrs: Vec<thread::JoinHandle<()>> = vec![];
    let mut stderr_thrs: Vec<thread::JoinHandle<()>> = vec![];
    let mut waiter_thrs: Vec<thread::JoinHandle<()>> = vec![];

    // tasks_status: the value used to store tasks's execution status
    let tasks_status = Arc::new(AtomicI32::new(TASKS_WAITING));
    let tasks_status_init = |tasks_status: &Arc<AtomicI32>, val: i32| {
        if TASKS_WAITING == tasks_status.load(Ordering::SeqCst) {
            tasks_status.store(val, Ordering::SeqCst);
        }
    };

    // children: spawn all tasks
    let children: HashMap<usize, (String, Arc<SharedChild>)> = HashMap::with_capacity(tasks.len());
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
        let waiter_task_name = task_name.clone();
        let waiter_child = Arc::clone(&child);
        let waiter_tasks_status = Arc::clone(&tasks_status);
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
                    tasks_status_init(&waiter_tasks_status, TASKS_FAILURE);
                }
                None => {
                    let line = "task terminated".to_string();
                    log::info!("{:>w$} {}", waiter_task_name, line.yellow(), w = maxwidth);
                    tasks_status_init(&waiter_tasks_status, TASKS_FAILURE);
                }
            }
        }));

        // child: append to children
        children.pin().insert(task_id, (task.name.clone(), child));
    }

    // retval: the value used to store tasks's execution failure reason
    let retval = Arc::new(Mutex::new(String::new()));
    let retval_init = |retval: &Arc<Mutex<String>>, errmsg: &str| {
        let mut retval = retval.lock().expect("failed to lock data");
        if (*retval).is_empty() {
            *retval = errmsg.to_string();
        };
    };

    // watcher: the function `watcher_reap` used to cleanup unfinished tasks
    let watcher_reap_retval = Arc::clone(&retval);
    let watcher_reap = move || {
        for (_task_id, (task_name, child)) in &children.pin() {
            // unfinished tasks: force kill
            let status = child.try_wait().expect("failed to try wait");
            if status.is_none() {
                child.kill().expect("failed to kill command");
                continue;
            }

            // finished tasks: store tasks's execution result
            let code = status.expect("Option::unwrap()").code();
            match code {
                Some(0) => {}
                Some(c) => {
                    let e = format!("failed to run task `{}`, exited with code {}", task_name, c);
                    retval_init(&watcher_reap_retval, &e);
                }
                None => {
                    let e = format!("failed to run task `{}`, terminated by signal", task_name);
                    retval_init(&watcher_reap_retval, &e);
                }
            }
        }
    };

    // watcher: periodically check tasks's execution status
    let watcher_tasks_status = Arc::clone(&tasks_status);
    let watcher_retval = Arc::clone(&retval);
    let watcher_thr = thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(167));

        // terminated by user
        if let Some(errmsg) = watched() {
            retval_init(&watcher_retval, &errmsg);
            watcher_reap();
            break;
        }

        // terminated by taskmgr
        let status = watcher_tasks_status.load(Ordering::SeqCst);
        match status {
            TASKS_WAITING => {
                continue;
            }
            TASKS_SUCCESS => {
                break;
            }
            TASKS_FAILURE => {
                watcher_reap();
                break;
            }
            _ => unreachable!(),
        }
    });

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

    // watcher: stop itself
    tasks_status_init(&tasks_status, TASKS_SUCCESS);
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

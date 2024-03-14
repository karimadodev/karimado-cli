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
                _ => _ = waiter_tx.send(1),
            }
        }));

        // child: append to children
        children.pin().insert(task_id, (task.name.clone(), child));
    }

    // retval:
    let retval = Arc::new(Mutex::new(String::new()));

    // watcher: collect all tasks's result -> retval
    let watcher_retval = Arc::clone(&retval);
    let watcher_reap = move || {
        for (_task_id, (task_name, child)) in &children.pin() {
            let status = child.try_wait().expect("failed to try wait");

            // unfinished tasks: force kill
            if status.is_none() {
                let watcher_task_name = colored_task_name(task_name);
                let line = format!("terminating `{}`", task_name);
                log::info!("{:>w$} {}", watcher_task_name, line.yellow(), w = maxwidth);
                child.kill().expect("failed to kill command");
                continue;
            }

            // finished tasks: succeed/failed
            let code = status.expect("Option::unwrap()").code().unwrap_or(-1);
            if code != 0 {
                let watcher_task_name = colored_task_name(task_name);
                let line = format!("failed to run task `{}`: exit code {}", task_name, code);
                log::info!("{:>w$} {}", watcher_task_name, line.red(), w = maxwidth);
                *watcher_retval.lock().expect("failed to lock data") = line.to_string();
            }
        }
    };
    let watcher_thr = thread::spawn(move || {
        // reason:
        //   0: all tasks succeed
        //   1: one of the tasks had failed
        //   2: received Ctrl-C signal
        let reason = rx.recv().expect("failed to recv");
        match reason {
            0 => {}
            1 => watcher_reap(),
            2 => watcher_reap(),
            _ => unreachable!(),
        }
    });

    // ctrlc:
    let ctrlc_tx = tx.clone();
    ctrlc::set_handler(move || _ = ctrlc_tx.send(2)).expect("failed to set Ctrl-C handler");

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
    _ = tx.send(0);
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

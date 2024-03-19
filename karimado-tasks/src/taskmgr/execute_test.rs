use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use super::*;
use crate::build_task;

#[test]
fn ok() {
    let tasks: Vec<Task> = vec![
        build_task!(command: "ruby -e 'sleep(1)'"),
        build_task!(command: "ruby -e 'sleep(1)'"),
    ];

    let now = Instant::now();
    let r = execute(&tasks, || None);
    let elapsed = now.elapsed();

    assert!(r.is_ok());
    assert!(elapsed > std::time::Duration::from_secs(2));
}

#[test]
fn err_command_not_found() {
    let tasks: Vec<Task> = vec![
        build_task!(command: "command404"),
        build_task!(command: "ruby -e 'sleep(1)'"),
    ];

    let r = execute(&tasks, || None);
    assert!(r.is_err());
    assert!(matches!(r, Err(Error::TaskRunFailed(_))));

    let e = r.unwrap_err().to_string();
    assert!(e.clone().contains("failed to run task"));
    assert!(e.clone().contains("exited with code"));
}

#[test]
fn err_exit_code_is_nonzero() {
    let tasks: Vec<Task> = vec![
        build_task!(command: "ruby -e 'exit(77)'"),
        build_task!(command: "ruby -e 'sleep(1)'"),
    ];

    let r = execute(&tasks, || None);
    assert!(r.is_err());
    assert!(matches!(r, Err(Error::TaskRunFailed(_))));

    let e = r.unwrap_err().to_string();
    assert!(e.clone().contains("failed to run task"));
    assert!(e.clone().contains("exited with code 77"));
}

#[test]
fn err_timeout() {
    let tasks: Vec<Task> = vec![
        build_task!(command: "ruby -e 'sleep(1)'"),
        build_task!(command: "ruby -e 'sleep(1)'"),
    ];

    let terminated = Arc::new(AtomicBool::new(false));
    let timeout_terminated = Arc::clone(&terminated);
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(500));
        timeout_terminated.store(true, Ordering::SeqCst);
    });

    let r = execute(&tasks, move || {
        if terminated.load(Ordering::SeqCst) {
            Some("timeout exceeded".to_string())
        } else {
            None
        }
    });
    assert!(r.is_err());
    assert!(matches!(r, Err(Error::TaskRunFailed(_))));

    let e = r.unwrap_err().to_string();
    assert!(e.clone().contains("timeout exceeded"));
}

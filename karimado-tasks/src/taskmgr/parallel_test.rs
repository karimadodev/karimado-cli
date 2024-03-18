use super::*;
use crate::build_task;

#[test]
fn ok() {
    let tasks: Vec<Task> = vec![
        build_task!(command: "ruby -e 'sleep(1)'"),
        build_task!(command: "ruby -e 'sleep(1)'"),
    ];

    let now = std::time::Instant::now();
    let r = execute(&tasks);
    let elapsed = now.elapsed();

    assert!(r.is_ok());
    assert!(elapsed < std::time::Duration::from_secs(2));
}

#[test]
fn err_command_not_found() {
    let tasks: Vec<Task> = vec![
        build_task!(command: "command404"),
        build_task!(command: "ruby -e 'sleep(1)'"),
    ];

    let r = execute(&tasks);
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

    let r = execute(&tasks);
    assert!(r.is_err());
    assert!(matches!(r, Err(Error::TaskRunFailed(_))));

    let e = r.unwrap_err().to_string();
    assert!(e.clone().contains("failed to run task"));
    assert!(e.clone().contains("exited with code 77"));
}

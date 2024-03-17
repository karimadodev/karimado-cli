use super::*;
use crate::build_task;

#[test]
fn ok() {
    let tasks: Vec<Task> = vec![build_task!(command: "cargo version")];

    let r = execute(&tasks, false);
    assert!(r.is_ok());
}

#[test]
fn err_command_not_found() {
    let tasks: Vec<Task> = vec![build_task!(command: "command404")];

    let r = execute(&tasks, false);
    assert!(r.is_err());
    assert!(matches!(r, Err(Error::TaskRunFailed(_))));

    let e = r.unwrap_err().to_string();
    assert!(e.clone().contains("failed to run task"));
    assert!(e.clone().contains("exited with code"));
}

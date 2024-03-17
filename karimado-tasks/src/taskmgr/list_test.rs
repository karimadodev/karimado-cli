use super::*;
use crate::build_task;

#[test]
fn ok() {
    let tasks: Vec<Task> = vec![
        build_task!(name: "cargo"),
        build_task!(name: "cargo:new"),
        build_task!(name: "cargo:install"),
        build_task!(name: "cargo:run"),
        build_task!(name: "cargo:run:test"),
        build_task!(name: "cargo:build"),
    ];
    list(&tasks);
}

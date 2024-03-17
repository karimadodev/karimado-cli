use super::*;

#[test]
fn ok() {
    let tasks_count = 4;
    let tasks: Vec<Task> = std::iter::repeat(Default::default())
        .take(tasks_count)
        .collect();
    list(&tasks);
}

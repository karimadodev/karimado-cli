fn main() -> Result<(), karimado_tasks::Error> {
    let args: Vec<_> = std::env::args().collect();
    let taskmgr = karimado_tasks::TaskMgr::builder()
        .current_dir(&std::env::current_dir().unwrap())
        .taskfile("examples/karimado/tasks.toml")
        .build()
        .unwrap();
    taskmgr.parallel_execute(&args[1..], || None)
}

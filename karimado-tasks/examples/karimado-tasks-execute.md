# taskmgr#execute

Executes tasks provided on command line:

```rust
fn main() -> Result<(), karimado_tasks::Error> {
    env_logger::builder()
        .format_level(false)
        .format_target(false)
        .format_timestamp(None)
        .filter_level(log::LevelFilter::Info)
        .init();

    let args: Vec<_> = std::env::args().collect();
    let taskmgr = karimado_tasks::TaskMgr::builder()
        .current_dir(&std::env::current_dir().unwrap())
        .taskfile("examples/karimado/tasks.toml")
        .build()
        .unwrap();
    taskmgr.execute(&args[1..], || None)
}
```


## Invoke one task

```console
$ karimado-tasks-execute ruby:true
-> ruby -e 'exit(0)'


```


## Invoke multiple tasks

```console
$ karimado-tasks-execute ruby:true ruby:true ruby:true
-> ruby -e 'exit(0)'

-> ruby -e 'exit(0)'

-> ruby -e 'exit(0)'


```


## Immediately terminated if task failed

```console
$ karimado-tasks-execute ruby:true ruby:false ruby:true
? 1
-> ruby -e 'exit(0)'

-> ruby -e 'exit(1)'

Error: TaskRunFailed("failed to run task `ruby:false`, exited with code 1")

```

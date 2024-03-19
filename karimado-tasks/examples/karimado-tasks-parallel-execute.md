# taskmgr#parallel_execute

Executes tasks provided on command line in parallel:

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
    taskmgr.parallel_execute(&args[1..], || None)
}
```

## Invoke one task

```console
$ karimado-tasks-parallel-execute true
 true | -> ruby -e 'exit(0)'
 true | task finished

```

## Invoke multiple tasks

```console
$ karimado-tasks-parallel-execute true true true
 true | -> ruby -e 'exit(0)'
 true | -> ruby -e 'exit(0)'
 true | -> ruby -e 'exit(0)'
 true | task finished
 true | task finished
 true | task finished

```

## Immediately terminated if task failed

```console
$ karimado-tasks-parallel-execute sleep1 false sleep1
? 1
 sleep1 | -> ruby -e 'sleep(1)'
  false | -> ruby -e 'exit(1)'
 sleep1 | -> ruby -e 'sleep(1)'
  false | task exited with code 1
 sleep1 | task terminated
 sleep1 | task terminated
Error: TaskRunFailed("failed to run task `false`, exited with code 1")

```

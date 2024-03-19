# taskmgr#parallel_execute

Executes tasks provided on command line in parallel:

[taskmgr-parallel-execute.rs](./taskmgr-parallel-execute.rs)

## Invoke one task

```console
$ taskmgr-parallel-execute true
 true | -> ruby -e 'exit(0)'
 true | task finished

```

## Invoke multiple tasks

```console
$ taskmgr-parallel-execute true true true
 true | -> ruby -e 'exit(0)'
 true | -> ruby -e 'exit(0)'
 true | -> ruby -e 'exit(0)'
 true | task finished
 true | task finished
 true | task finished

```

## Immediately terminated if task failed

```console
$ taskmgr-parallel-execute sleep1 false sleep1
? 1
 sleep1 | -> ruby -e 'sleep(1)'
  false | -> ruby -e 'exit(1)'
 sleep1 | -> ruby -e 'sleep(1)'
  false | task exited with code 1
 sleep1 | task [..]
 sleep1 | task [..]
Error: TaskRunFailed("failed to run task `false`, exited with code 1")

```

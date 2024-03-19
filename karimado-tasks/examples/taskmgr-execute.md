# taskmgr#execute

Executes tasks provided on command line:

[taskmgr-execute.rs](./taskmgr-execute.rs)

## Invoke one task

```console
$ taskmgr-execute true
-> ruby -e 'exit(0)'


```

## Invoke multiple tasks

```console
$ taskmgr-execute true true true
-> ruby -e 'exit(0)'

-> ruby -e 'exit(0)'

-> ruby -e 'exit(0)'


```

## Immediately terminated if task failed

```console
$ taskmgr-execute sleep1 false sleep1
? 1
-> ruby -e 'sleep(1)'

-> ruby -e 'exit(1)'

Error: TaskRunFailed("failed to run task `false`, exited with code 1")

```

# taskmgr#parallel_execute

Executes tasks provided on command line in parallel

[taskmgr-parallel-execute.rs](./taskmgr-parallel-execute.rs)

## Invoke one task

```console
$ taskmgr-parallel-execute true
 true.1 | -> ruby -e 'exit(0)'
 true.1 | <> task finished

```

## Invoke multiple tasks

```console
$ taskmgr-parallel-execute sleep1 true sleep2
 sleep1.1 | -> ruby -e 'sleep(1)'
   true.2 | -> ruby -e 'exit(0)'
 sleep2.3 | -> ruby -e 'sleep(2)'
   true.2 | <> task finished
 sleep1.1 | <> task finished
 sleep2.3 | <> task finished

```

## Immediately terminated if task failed

```console
$ taskmgr-parallel-execute false sleep1
? 1
  false.1 | -> ruby -e 'exit(1)'
 sleep1.2 | -> ruby -e 'sleep(1)'
  false.1 | <> task exited with code 1
 sleep1.2 | <> task [..]
Error: TaskRunError("failed to run task `false`, exited with code 1")

```

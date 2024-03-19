# taskmgr#execute

Executes tasks provided on command line


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

```
$ karimado-tasks-execute ruby:true ruby:false ruby:true
? 1
-> ruby -e 'exit(0)'

-> ruby -e 'exit(1)'

Error: TaskRunFailed("failed to run task `ruby:false`, exited with code 1")

```

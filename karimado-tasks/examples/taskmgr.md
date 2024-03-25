# taskmgr

Run a defined task

[taskmgr.rs](./taskmgr.rs)

## --help

```console
$ taskmgr --help
Library for running a command defined in the taskfile.

Usage: taskmgr[EXE] [OPTIONS] [TASK]... [-- <ARGS>...]

Arguments:
  [TASK]...  Task Name [default: default]
  [ARGS]...  Task Args

Options:
  -l, --list      Lists tasks with description of current taskfile
  -p, --parallel  Executes tasks provided on command line in parallel
  -h, --help      Print help
  -V, --version   Print version

```

## --list

```console
$ taskmgr --list
Available tasks for this project:
* cargo:build   # Compile the current package
* cargo:fmt     # Formats all files
* cargo:install # Install a Rust binary
* cargo:new     # Create a new cargo package
* cargo:run     # Run a binary or example of the local package
* false         # ruby -e 'exit(1)'
* sleep1        # ruby -e 'sleep(1)'
* sleep2        # ruby -e 'sleep(2)'
* sleepn        # ruby -e 'sleep(n)'
* true          # ruby -e 'exit(0)'

```

## --parallel

```console
$ taskmgr -p sleep1 true sleepn -- 2
 sleep1.1 | -> ruby -e 'sleep(1)'
   true.2 | -> ruby -e 'exit(0)'
 sleepn.3 | -> ruby -e 'sleep(2)'
   true.2 | <> task finished
 sleep1.1 | <> task finished
 sleepn.3 | <> task finished

```

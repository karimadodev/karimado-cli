# taskmgr

Run a defined task:

[taskmgr.rs](./taskmgr.rs)

## --help

```console
$ taskmgr --help
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
* sleepn        # ruby -e 'sleep(n)'
* true          # ruby -e 'exit(0)'

```

## --parallel

```console
$ taskmgr -p sleepn true sleepn -- 2
 sleepn | -> ruby -e 'sleep(2)'
   true | -> ruby -e 'exit(0)'
 sleepn | -> ruby -e 'sleep(2)'
   true | task finished
 sleepn | task finished
 sleepn | task finished

```

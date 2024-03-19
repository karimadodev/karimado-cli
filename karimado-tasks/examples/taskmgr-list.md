# taskmgr#list

Lists tasks with description of current taskfile:

[taskmgr-list.rs](./taskmgr-list.rs)

## List tasks

```console
$ taskmgr-list
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

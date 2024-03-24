# karimado-cli#karimado

## Installation

With Rust's package manager [cargo](https://github.com/rust-lang/cargo), you can install `karimado` via

```sh
cargo install karimado
```

## Usage

```console
$ karimado --help
Usage: karimado [OPTIONS] <COMMAND>

Commands:
  new               Create a new Karimado application at the path you specify
  run               Run a defined task
  build             Parse and transform Karimado Modeling Language files
  scaffold:install  Install scaffolding files in a specific location

Options:
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help
  -V, --version     Print version

```

More examples can be found in [examples](./examples).

## License

The crate is available as open source under the terms of the [AGPL-3.0-only License](./LICENSE).

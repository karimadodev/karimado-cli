# karimado-cli#karimado-source-dl

## Installation

Add following line to `Cargo.toml` file:

```toml
[dependencies]
karimado-source-dl = "0.0"
```

## Usage

```rust
fn main() {
    let url = "git+https://github.com/karimadodev/karimado-cli.git#8afbd6d";
    let path = karimado_source_dl::Downloader::new().download(url).unwrap();
    println!("{}", path.display());
}
```

More examples can be found in [examples](./examples).

## License

The crate is available as open source under the terms of the [LGPL-3.0-only License](./LICENSE).

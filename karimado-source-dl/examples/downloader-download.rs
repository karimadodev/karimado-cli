fn main() {
    for url in vec![
        "git+https://github.com/karimadodev/karimado-cli.git#8afbd6d",
        "https://github.com/karimadodev/karimado-cli/raw/main/karimado-source-dl/tests/fixtures/archive/hello-world.tar.gz",
        "tests/fixtures/archive/hello-world.zip",
    ] {
        let path = karimado_source_dl::Downloader::new().download(url).unwrap();
        println!("Downloading source code...");
        println!("  source: {}", url);
        println!("  destination: {}", path.display());
        println!("");
    }
}

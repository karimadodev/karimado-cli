use super::*;

#[test]
fn ok() {
    let tmpdir = tempfile::TempDir::new().unwrap();
    let downloads_path = tmpdir.path();
    let url = Url::parse("https://github.com/karimadodev/karimado-cli/raw/main/karimado-source-dl/tests/fixtures/archive/hello-world.tar.gz").unwrap();
    let r = download(&url, downloads_path);
    assert!(r.is_ok());
}

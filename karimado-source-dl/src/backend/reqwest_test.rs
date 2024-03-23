use super::*;

#[test]
fn ok() {
    let tmpdir = tempfile::TempDir::new().unwrap();
    let downloads_path = tmpdir.path();
    let url = Url::parse_with_quirks_mode("https://github.com/karimadodev/karimado-cli/raw/main/karimado-source-dl/tests/fixtures/archive/hello-world.tar.gz", None).unwrap();
    let r = download(&url, downloads_path);
    assert!(r.is_ok());

    let target_path = r.unwrap();
    assert!(target_path.join("src").is_dir());
    assert!(target_path.join("src/main.rs").is_file());
    assert!(target_path.join("Cargo.lock").is_file());
    assert!(target_path.join("Cargo.toml").is_file());
    assert!(target_path.join(".gitignore").is_file());
}

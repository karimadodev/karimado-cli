use super::*;

#[test]
fn ok_dir() {
    let tmpdir = tempfile::TempDir::new().unwrap();
    let downloads_path = tmpdir.path();
    let filename = "tests/fixtures/archive/hello-world";
    let url = Url::parse_with_quirks_mode(filename, None).unwrap();
    let r = download(&url, downloads_path);
    assert!(r.is_ok());

    let path = r.unwrap();
    assert!(path.join("src").is_dir());
    assert!(path.join("src/main.rs").is_file());
    assert!(path.join("Cargo.lock").is_file());
    assert!(path.join("Cargo.toml").is_file());
    assert!(path.join(".gitignore").is_file());
}

#[test]
fn ok_file() {
    let tmpdir = tempfile::TempDir::new().unwrap();
    let downloads_path = tmpdir.path();
    let filename = "tests/fixtures/archive/hello-world.tar.gz";
    let url = Url::parse_with_quirks_mode(filename, None).unwrap();
    let r = download(&url, downloads_path);
    assert!(r.is_ok());

    let path = r.unwrap();
    assert!(path.join("src").is_dir());
    assert!(path.join("src/main.rs").is_file());
    assert!(path.join("Cargo.lock").is_file());
    assert!(path.join("Cargo.toml").is_file());
    assert!(path.join(".gitignore").is_file());
}

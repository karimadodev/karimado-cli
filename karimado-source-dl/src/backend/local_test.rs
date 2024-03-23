use super::*;

#[test]
fn ok_dir() {
    let tmpdir = tempfile::TempDir::new().unwrap();
    let downloads_path = tmpdir.path();
    let filename = "tests/fixtures/archive/hello-world";
    let filepath = std::env::current_dir().unwrap().join(filename);
    let url = Url::parse(&format!("file://{}", filepath.to_str().unwrap())).unwrap();
    let r = download(&url, downloads_path);
    assert!(r.is_ok());

    let target_path = r.unwrap();
    assert!(target_path.join("src").is_dir());
    assert!(target_path.join("src/main.rs").is_file());
    assert!(target_path.join("Cargo.lock").is_file());
    assert!(target_path.join("Cargo.toml").is_file());
    assert!(target_path.join(".gitignore").is_file());
}

#[test]
fn ok_file() {
    let tmpdir = tempfile::TempDir::new().unwrap();
    let downloads_path = tmpdir.path();
    let filename = "tests/fixtures/archive/hello-world.tar.gz";
    let filepath = std::env::current_dir().unwrap().join(filename);
    let url = Url::parse(&format!("file://{}", filepath.to_str().unwrap())).unwrap();
    let r = download(&url, downloads_path);
    assert!(r.is_ok());

    let target_path = r.unwrap();
    assert!(target_path.join("src").is_dir());
    assert!(target_path.join("src/main.rs").is_file());
    assert!(target_path.join("Cargo.lock").is_file());
    assert!(target_path.join("Cargo.toml").is_file());
    assert!(target_path.join(".gitignore").is_file());
}

use super::*;

#[test]
fn ok() {
    let tmpdir = tempfile::TempDir::new().unwrap();
    let target_path = tmpdir.path();
    let archive_path = "tests/fixtures/archive/hello-world.tar.gz";
    let r = decompress(&Path::new(archive_path), target_path);
    assert!(r.is_ok());

    assert!(target_path.join("src").is_dir());
    assert!(target_path.join("src/main.rs").is_file());
    assert!(target_path.join("Cargo.lock").is_file());
    assert!(target_path.join("Cargo.toml").is_file());
    assert!(target_path.join(".gitignore").is_file());
}

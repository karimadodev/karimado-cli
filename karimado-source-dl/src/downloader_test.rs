use std::env;
use tempfile::TempDir;

use super::*;

#[test]
fn ok() {
    let downloader = Downloader::new();
    let r = downloader.download("tests/fixtures/archive/hello-world");
    assert!(r.is_ok());

    let path = r.unwrap();
    assert!(path.starts_with(env::temp_dir()));

    let dirname = path.strip_prefix(env::temp_dir()).unwrap();
    assert!(dirname.to_str().unwrap().starts_with("kari"));
}

#[test]
fn ok_current_dir() {
    let mut downloader = Downloader::new();
    downloader.current_dir(&env::current_dir().unwrap().join("tests/fixtures"));

    let r = downloader.download("archive/hello-world.tar.gz");
    assert!(r.is_ok());
}

#[test]
fn ok_downloads_dir() {
    let tmpdir = TempDir::new().unwrap();
    let mut downloader = Downloader::new();
    downloader.downloads_dir(tmpdir.path());

    let r = downloader.download("tests/fixtures/archive/hello-world.zip");
    assert!(r.is_ok());

    let path = r.unwrap();
    assert!(path.starts_with(tmpdir.path()));

    let dirname = path.strip_prefix(tmpdir.path()).unwrap();
    assert!(dirname.to_str().unwrap().starts_with("kari"));
}

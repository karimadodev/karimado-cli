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
fn ok_downloads_path() {
    let tmpdir = TempDir::new().unwrap();
    let mut downloader = Downloader::new();
    downloader.downloads_path(tmpdir.path());

    let r = downloader.download("tests/fixtures/archive/hello-world.tar.gz");
    assert!(r.is_ok());

    let path = r.unwrap();
    assert!(path.starts_with(tmpdir.path()));

    let dirname = path.strip_prefix(tmpdir.path()).unwrap();
    assert!(dirname.to_str().unwrap().starts_with("kari"));
}

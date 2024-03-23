use std::env;
use tempfile::TempDir;

use super::*;

#[test]
fn ok() {
    let downloader = Downloader::new();
    let download = Download::new("tests/fixtures/archive/hello-world");
    let r = downloader.download(&download);
    assert!(r.is_ok());

    let target_path = r.unwrap();
    assert!(target_path.starts_with(env::temp_dir()));
}

#[test]
fn ok_downloads_path() {
    let tmpdir = TempDir::new().unwrap();
    let mut downloader = Downloader::new();
    downloader.downloads_path(tmpdir.path());

    let download = Download::new("tests/fixtures/archive/hello-world");
    let r = downloader.download(&download);
    assert!(r.is_ok());

    let target_path = r.unwrap();
    assert!(target_path.starts_with(tmpdir));
}

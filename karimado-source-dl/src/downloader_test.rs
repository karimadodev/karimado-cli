use std::env;
use tempfile::TempDir;

use super::*;
use crate::contrib;

#[test]
fn ok() {
    let download = Download::new("tests/fixtures/archive/hello-world");
    let downloader = Downloader::new();
    let r = downloader.download(&download);
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

    let download = Download::new("tests/fixtures/archive/hello-world");
    let r = downloader.download(&download);
    assert!(r.is_ok());

    let path = r.unwrap();
    assert!(path.starts_with(tmpdir.path()));

    let dirname = path.strip_prefix(tmpdir.path()).unwrap();
    assert!(dirname.to_str().unwrap().starts_with("kari"));
}

#[test]
fn ok_download_dirname_absolute_path() {
    let tmpdir = TempDir::new().unwrap();
    let mut download = Download::new("tests/fixtures/archive/hello-world");
    download.dirname(tmpdir.path().to_str().unwrap());

    let downloader = Downloader::new();
    let r = downloader.download(&download);
    assert!(r.is_ok());

    let path = r.unwrap();
    assert_eq!(path, tmpdir.path());
}

#[test]
fn ok_download_dirname_relative_path() {
    let dirname = contrib::uuid();
    let mut download = Download::new("tests/fixtures/archive/hello-world");
    download.dirname(&dirname);

    let downloader = Downloader::new();
    let r = downloader.download(&download);
    assert!(r.is_ok());

    let path = r.unwrap();
    assert_eq!(path, env::temp_dir().join(dirname));
}

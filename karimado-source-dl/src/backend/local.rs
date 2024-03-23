#[cfg(test)]
#[path = "local_test.rs"]
mod tests;

use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::{archive, contrib, error::*, Url};
use SourceDownloadErrorKind::IoError;

pub(crate) fn download(url: &Url, downloads_dir: &Path) -> Result<PathBuf> {
    let source = url.to_file_path().unwrap();
    let destination = downloads_dir.join(contrib::uuid());

    if source.is_dir() {
        sync(&source, &destination)?;
    } else {
        archive::decompress(&source, &destination)?;
    }

    Ok(destination)
}

fn sync(source: &Path, destination: &Path) -> Result<()> {
    for entry in WalkDir::new(source).into_iter().filter_map(|e| e.ok()) {
        let from = entry.path();
        let relpath = from.strip_prefix(source).expect("Path::strip_prefix()");
        let to = destination.join(relpath);

        if from.is_dir() {
            fs::create_dir_all(to).map_err(IoError)?;
        } else if from.is_file() {
            fs::copy(from, to).map_err(IoError)?;
        }
    }
    Ok(())
}

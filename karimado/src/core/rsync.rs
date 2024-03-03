use anyhow::Result;
use std::{fs, path::Path};
use walkdir::WalkDir;

use crate::contrib;

pub(crate) fn sync(source: &Path, target: &Path) -> Result<()> {
    for entry in WalkDir::new(source).into_iter().filter_map(|e| e.ok()) {
        let from = entry.path();
        let relpath = from.strip_prefix(source)?;
        let to = target.join(relpath);

        if from.is_dir() {
            sync_dir(from, &to, relpath)?;
        } else if from.is_file() {
            sync_file(from, &to, relpath)?;
        }
    }
    Ok(())
}

fn sync_dir(_from: &Path, to: &Path, relpath: &Path) -> Result<()> {
    if to.exists() {
        log::info!("{:>13}  {}", "exist", relpath.display());
    } else {
        fs::create_dir_all(to)?;
        log::info!("{:>13}  {}", "create", relpath.display());
    }
    Ok(())
}

fn sync_file(from: &Path, to: &Path, relpath: &Path) -> Result<()> {
    if to.exists() {
        if contrib::fs::is_file_identical(from, to)? {
            log::info!("{:>13}  {}", "identical", relpath.display());
        } else {
            fs::copy(from, to)?;
            log::info!("{:>13}  {}", "force", relpath.display());
        }
    } else {
        fs::copy(from, to)?;
        log::info!("{:>13}  {}", "create", relpath.display());
    }
    Ok(())
}

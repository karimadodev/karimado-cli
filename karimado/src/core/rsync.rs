use anyhow::Result;
use std::{fs, path::Path};
use walkdir::WalkDir;

pub(crate) fn sync(source: &Path, target: &Path) -> Result<()> {
    for entry in WalkDir::new(source).into_iter().filter_map(|e| e.ok()) {
        let from = entry.path();
        let from_relpath = from.strip_prefix(source)?;
        let to = target.join(from_relpath);

        if from.is_dir() {
            if to.exists() {
                log::debug!(" exist {}", from_relpath.display());
            } else {
                fs::create_dir_all(&to)?;
                log::debug!("create {}", from_relpath.display());
            }
        } else if from.is_file() {
            if to.exists() {
                fs::copy(from, to)?;
                log::debug!(" force {}", from_relpath.display());
            } else {
                fs::copy(from, to)?;
                log::debug!("create {}", from_relpath.display());
            }
        }
    }

    Ok(())
}

use anyhow::Result;
use colored::Colorize;
use std::{fs, path::Path};
use walkdir::WalkDir;

use crate::contrib;

pub(crate) fn sync(source: &Path, target: &Path) -> Result<(i32, i32, i32, i32)> {
    let mut added = 0;
    let mut removed = 0;
    let mut identical = 0;
    let mut overwritten = 0;

    for entry in WalkDir::new(source).into_iter().filter_map(|e| e.ok()) {
        let from = entry.path();
        let relpath = from.strip_prefix(source)?;
        let to = target.join(relpath);

        if from.is_dir() {
            sync_dir(from, &to, relpath)?;
        } else if from.is_file() {
            let (a, r, i, o) = sync_file(from, &to, relpath)?;
            added += a;
            removed += r;
            identical += i;
            overwritten += o;
        }
    }

    Ok((added, removed, identical, overwritten))
}

fn sync_dir(_from: &Path, to: &Path, relpath: &Path) -> Result<()> {
    if to.exists() {
        log::info!("{:>13}  {}", "exist".blue(), relpath.display());
    } else {
        fs::create_dir_all(to)?;
        log::info!("{:>13}  {}", "create".green(), relpath.display());
    }
    Ok(())
}

fn sync_file(from: &Path, to: &Path, relpath: &Path) -> Result<(i32, i32, i32, i32)> {
    if to.exists() {
        if contrib::fs::is_file_identical(from, to)? {
            log::info!("{:>13}  {}", "identical".blue(), relpath.display());
            Ok((0, 0, 1, 0))
        } else {
            fs::copy(from, to)?;
            log::info!("{:>13}  {}", "force".yellow(), relpath.display());
            Ok((0, 0, 0, 1))
        }
    } else {
        fs::copy(from, to)?;
        log::info!("{:>13}  {}", "create".green(), relpath.display());
        Ok((1, 0, 0, 0))
    }
}

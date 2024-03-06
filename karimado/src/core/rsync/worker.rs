use anyhow::Result;
use colored::Colorize;
use std::{
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

use super::options::Options;
use crate::contrib;

pub(crate) struct Worker {
    source: PathBuf,
    destination: PathBuf,
}

impl Worker {
    pub(crate) fn new(source: &Path, destination: &Path) -> Self {
        Self {
            source: source.to_path_buf(),
            destination: destination.to_path_buf(),
        }
    }

    pub(crate) fn sync(&self, options: &Options) -> Result<()> {
        let source = &self.source;
        let destination = &self.destination;
        let globset_include = options.globset_include()?;

        for entry in WalkDir::new(source).into_iter().filter_map(|e| e.ok()) {
            let from = entry.path();
            let relpath = from.strip_prefix(source)?;
            let to = destination.join(relpath);

            if globset_include.matches(relpath).is_empty() {
                continue;
            }

            if from.is_dir() {
                self.sync_dir(from, &to, relpath)?;
            } else if from.is_file() {
                self.sync_file(from, &to, relpath)?;
            }
        }

        Ok(())
    }

    fn sync_dir(&self, from: &Path, to: &Path, relpath: &Path) -> Result<()> {
        if to.exists() {
            self.sync_dir_exist(from, to, relpath)
        } else {
            self.sync_dir_create(from, to, relpath)
        }
    }

    fn sync_dir_exist(&self, _from: &Path, _to: &Path, relpath: &Path) -> Result<()> {
        log::info!("{:>13}  {}", "exist".blue(), relpath.display());
        Ok(())
    }

    fn sync_dir_create(&self, _from: &Path, to: &Path, relpath: &Path) -> Result<()> {
        fs::create_dir_all(to)?;
        log::info!("{:>13}  {}", "create".green(), relpath.display());
        Ok(())
    }

    fn sync_file(&self, from: &Path, to: &Path, relpath: &Path) -> Result<()> {
        if to.exists() {
            self.sync_file_exist(from, to, relpath)
        } else {
            self.sync_file_create(from, to, relpath)
        }
    }

    fn sync_file_exist(&self, from: &Path, to: &Path, relpath: &Path) -> Result<()> {
        if contrib::fs::is_file_identical(from, to)? {
            self.sync_file_identical(from, to, relpath)
        } else {
            self.sync_file_force(from, to, relpath)
        }
    }

    fn sync_file_identical(&self, _from: &Path, _to: &Path, relpath: &Path) -> Result<()> {
        log::info!("{:>13}  {}", "identical".blue(), relpath.display());
        Ok(())
    }

    fn sync_file_force(&self, from: &Path, to: &Path, relpath: &Path) -> Result<()> {
        fs::copy(from, to)?;
        log::info!("{:>13}  {}", "force".yellow(), relpath.display());
        Ok(())
    }

    fn sync_file_create(&self, from: &Path, to: &Path, relpath: &Path) -> Result<()> {
        if let Some(parent) = to.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(from, to)?;
        log::info!("{:>13}  {}", "create".green(), relpath.display());
        Ok(())
    }
}

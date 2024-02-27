use anyhow::Result;
use clap::{Args, ValueEnum};
use git2::Repository;
use path_absolutize::Absolutize;
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::assets;

#[derive(Args)]
pub(crate) struct NewCommand {
    /// Initialize a new repository for the given version control system
    #[arg(long, value_enum, default_value = "git")]
    vcs: VersionControl,

    #[arg(default_value = ".")]
    path: PathBuf,
}

#[derive(Clone, ValueEnum)]
enum VersionControl {
    Git,
    None,
}

impl NewCommand {
    pub(crate) fn execute(&self) -> Result<()> {
        let path = self.path.absolutize()?;
        if path.exists() && path.read_dir()?.next().is_some() {
            anyhow::bail!("destination {:?} is not empty", path)
        }

        fs::create_dir_all(&path)?;
        self.initialize_scaffold(&path)?;
        self.initialize_vcs_repository(&path)?;

        Ok(())
    }

    fn initialize_scaffold(&self, path: &Path) -> Result<()> {
        assets::copy("template/karimado.toml", &path.join("karimado.toml"))?;
        assets::copy("template/README.md", &path.join("README.md"))?;
        Ok(())
    }

    fn initialize_vcs_repository(&self, path: &Path) -> Result<()> {
        match self.vcs {
            VersionControl::Git => {
                assets::copy("template/.gitignore", &path.join(".gitignore"))?;
                Repository::init(path)?;
            }
            _ => {}
        };

        Ok(())
    }
}

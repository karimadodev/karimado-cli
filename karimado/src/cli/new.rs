use anyhow::Result;
use clap::{Args, ValueEnum};
use colored::Colorize;
use git2::Repository;
use path_absolutize::Absolutize;
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{assets, contrib};

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
        if contrib::fs::is_dir_nonempty(&path)? {
            anyhow::bail!("destination {} is not empty", path.display())
        }

        log::info!("Initializing project in {}...", path.display());
        log::info!("");
        fs::create_dir_all(&path)?;
        self.initialize_scaffold(&path)?;
        self.initialize_vcs_repository(&path)?;

        log::info!("Done. Now run:");
        log::info!("");
        log::info!("  {}", format!("cd {}", self.path.display()).green());
        log::info!("  {}", "karimado scaffold:install".green());
        log::info!("");
        Ok(())
    }

    fn initialize_scaffold(&self, path: &Path) -> Result<()> {
        for file in ["karimado/tasks.toml", "karimado.toml", "README.md"] {
            assets::copy(&format!("template/{}", file), &path.join(file))?;
        }
        Ok(())
    }

    fn initialize_vcs_repository(&self, path: &Path) -> Result<()> {
        if let VersionControl::Git = self.vcs {
            assets::copy("template/.gitignore", &path.join(".gitignore"))?;
            Repository::init(path)?;
        };
        Ok(())
    }
}

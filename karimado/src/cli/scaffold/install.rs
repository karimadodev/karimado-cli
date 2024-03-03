use anyhow::Result;
use clap::Args;
use colored::Colorize;
use std::{fs, path::Path};
use strum::Display;
use url::Url;

use crate::{
    config::{self, Config},
    contrib,
    core::{download, rsync},
};

#[derive(Args)]
pub(crate) struct InstallCommand {
    /// Overwrite files that already exist
    #[arg(long, short)]
    force: bool,
}

#[derive(Display)]
enum ScaffoldKind {
    #[strum(serialize = "web")]
    Web,
    #[strum(serialize = "server")]
    Server,
}

impl InstallCommand {
    pub(crate) fn execute(&self) -> Result<()> {
        let root_path = contrib::cli::root_path()?;
        let config_file_path = contrib::cli::config_file_path(&root_path);
        let config = config::from_config_file(&config_file_path)?;

        log::info!("Scaffolding project in {}...", root_path.display());
        log::info!("");
        fs::create_dir_all(root_path.join("tmp/cache/scaffolds"))?;
        fs::create_dir_all(root_path.join("tmp/downloads"))?;

        self.download_web_scaffold(&config, &root_path)?;
        self.download_server_scaffold(&config, &root_path)?;

        self.install_web_scaffold(&config, &root_path)?;
        self.install_server_scaffold(&config, &root_path)?;

        Ok(())
    }

    fn download_web_scaffold(&self, config: &Config, root_path: &Path) -> Result<()> {
        match &config.workspace.web {
            Some(value) => self.download_scaffold(
                ScaffoldKind::Web,
                &value.name,
                &value.scaffold.url,
                root_path,
            ),
            _ => Ok(()),
        }
    }

    fn download_server_scaffold(&self, config: &Config, root_path: &Path) -> Result<()> {
        match &config.workspace.server {
            Some(value) => self.download_scaffold(
                ScaffoldKind::Server,
                &value.name,
                &value.scaffold.url,
                root_path,
            ),
            _ => Ok(()),
        }
    }

    fn download_scaffold(
        &self,
        kind: ScaffoldKind,
        name: &str,
        url: &Url,
        root_path: &Path,
    ) -> Result<()> {
        log::info!("Downloading {} scaffold `{}`...", kind, name);

        let path = root_path.join("tmp/cache/scaffolds").join(name);
        if path.exists() {
            log::info!("Found cache in {}", path.display());
        } else {
            log::info!("Fetching from {}", url);
            let download_path = download::download(url, &root_path.join("tmp/downloads"))?;
            fs::rename(download_path, path)?;
        }

        log::info!("");
        Ok(())
    }

    fn install_web_scaffold(&self, config: &Config, root_path: &Path) -> Result<()> {
        match &config.workspace.web {
            Some(value) => self.install_scaffold(
                ScaffoldKind::Web,
                &value.name,
                &value.scaffold.template_path,
                root_path,
            ),
            _ => Ok(()),
        }
    }

    fn install_server_scaffold(&self, config: &Config, root_path: &Path) -> Result<()> {
        match &config.workspace.server {
            Some(value) => self.install_scaffold(
                ScaffoldKind::Server,
                &value.name,
                &value.scaffold.template_path,
                root_path,
            ),
            _ => Ok(()),
        }
    }

    fn install_scaffold(
        &self,
        kind: ScaffoldKind,
        name: &str,
        template_path: &str,
        root_path: &Path,
    ) -> Result<()> {
        log::info!(
            "Copying template files from {} scaffold `{}` to workspace...",
            kind,
            name
        );

        let source = root_path
            .join("tmp/cache/scaffolds")
            .join(name)
            .join(template_path);
        if !source.exists() {
            anyhow::bail!("template_path {} is not exists", source.display());
        }

        let target = root_path.join(kind.to_string());
        if contrib::fs::is_dir_nonempty(&target)? && !self.force {
            anyhow::bail!(
                "target {} is not empty, use the `--force` flag to re-install them",
                target.display()
            );
        }

        let (added, _, identical, overwritten) = rsync::sync(&source, root_path)?;
        log::info!(
            "Total: {}, {}, {}",
            format!("added {}", added).green(),
            format!("identical {}", identical).blue(),
            format!("overwritten {}", overwritten).yellow()
        );
        log::info!("");
        Ok(())
    }
}

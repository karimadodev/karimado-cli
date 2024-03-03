use anyhow::Result;
use clap::Args;
use std::{fs, path::Path};
use url::Url;

use crate::{
    config::{self, Config},
    contrib,
    core::{download, rsync},
};

#[derive(Args)]
pub(crate) struct InstallCommand {}

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
            Some(value) => self.download_scaffold(&value.name, &value.scaffold.url, root_path),
            _ => Ok(()),
        }
    }

    fn download_server_scaffold(&self, config: &Config, root_path: &Path) -> Result<()> {
        match &config.workspace.server {
            Some(value) => self.download_scaffold(&value.name, &value.scaffold.url, root_path),
            _ => Ok(()),
        }
    }

    fn download_scaffold(&self, name: &str, url: &Url, root_path: &Path) -> Result<()> {
        log::info!("Downloading scaffold {}...", name);

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
            Some(value) => {
                self.install_scaffold(&value.name, &value.scaffold.template_path, root_path)
            }
            _ => Ok(()),
        }
    }

    fn install_server_scaffold(&self, config: &Config, root_path: &Path) -> Result<()> {
        match &config.workspace.server {
            Some(value) => {
                self.install_scaffold(&value.name, &value.scaffold.template_path, root_path)
            }
            _ => Ok(()),
        }
    }

    fn install_scaffold(&self, name: &str, template_path: &str, root_path: &Path) -> Result<()> {
        log::info!("Copying template files...");

        let source = root_path
            .join("tmp/cache/scaffolds")
            .join(name)
            .join(template_path);
        if !source.exists() {
            anyhow::bail!("template_path {} is not exists", source.display());
        }

        rsync::sync(&source, root_path)?;
        log::info!("Copied.");
        log::info!("");
        Ok(())
    }
}

use anyhow::Result;
use clap::Args;
use std::{fs, path::Path};
use url::Url;

use crate::{
    cli::contrib,
    config::{self, Config},
    core::download,
};

#[derive(Args)]
pub(crate) struct InstallCommand {
    /// Overwrite files that already exist
    #[arg(long)]
    force: bool,
}

impl InstallCommand {
    pub(crate) fn execute(&self) -> Result<()> {
        let root_path = contrib::root_path()?;
        let config_file = root_path.join("karimado.toml");
        let config = config::from_config_file(&config_file)?;

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
            Some(val) => self.download_scaffold(&val.name, &val.scaffold.url, root_path),
            _ => Ok(()),
        }
    }

    fn download_server_scaffold(&self, config: &Config, root_path: &Path) -> Result<()> {
        match &config.workspace.server {
            Some(val) => self.download_scaffold(&val.name, &val.scaffold.url, root_path),
            _ => Ok(()),
        }
    }

    fn download_scaffold(&self, name: &str, url: &Url, root_path: &Path) -> Result<()> {
        let path = root_path.join("tmp/cache/scaffolds").join(name);
        if path.exists() {
            return Ok(());
        }

        let download_path = download::download(url, &root_path.join("tmp/downloads"))?;
        fs::rename(download_path, path)?;
        Ok(())
    }

    fn install_web_scaffold(&self, config: &Config, root_path: &Path) -> Result<()> {
        match &config.workspace.web {
            Some(val) => self.install_scaffold(&val.name, &val.scaffold.template_path, root_path),
            _ => Ok(()),
        }
    }

    fn install_server_scaffold(&self, config: &Config, root_path: &Path) -> Result<()> {
        match &config.workspace.server {
            Some(val) => self.install_scaffold(&val.name, &val.scaffold.template_path, root_path),
            _ => Ok(()),
        }
    }

    fn install_scaffold(&self, name: &str, template_path: &str, root_path: &Path) -> Result<()> {
        let _path = root_path
            .join("tmp/cache/scaffolds")
            .join(name)
            .join(template_path);
        Ok(())
    }
}

use anyhow::Result;
use clap::Args;
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
    #[strum(serialize = "server")]
    Server,
    #[strum(serialize = "web")]
    Web,
}

impl InstallCommand {
    pub(crate) fn execute(&self) -> Result<()> {
        let root_path = contrib::cli::root_path()?;
        let config_file_path = contrib::cli::config_file_path(&root_path);
        let config = config::from_config_file(&config_file_path)?;

        log::info!("Scaffolding project in {}...", root_path.display());
        log::info!("");
        _ = fs::remove_dir_all(root_path.join("tmp/downloads"));
        fs::create_dir_all(root_path.join("tmp/cache/scaffolds"))?;
        fs::create_dir_all(root_path.join("tmp/downloads"))?;

        self.download_server_scaffold(&config, &root_path)?;
        self.download_web_scaffold(&config, &root_path)?;

        self.install_server_scaffold(&config, &root_path)?;
        self.install_web_scaffold(&config, &root_path)?;

        Ok(())
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

    fn install_scaffold(
        &self,
        kind: ScaffoldKind,
        name: &str,
        template_pathname: &str,
        root_path: &Path,
    ) -> Result<()> {
        log::info!(
            "Copying template files from {} scaffold `{}` to workspace...",
            kind,
            name
        );

        let scaffold_path = root_path.join("tmp/cache/scaffolds").join(name);
        log::info!("  scaffold path: {}", scaffold_path.display());

        let template_path = scaffold_path.join(template_pathname);
        log::info!("  scaffold template path: {}", template_path.display());

        if !template_path.exists() {
            anyhow::bail!(
                "invalid {} scaffold format: template path `{}` does not exists",
                kind,
                template_pathname
            );
        }

        let source = template_path.join(kind.to_string());
        if !source.exists() {
            anyhow::bail!(
                "invalid {} scaffold format: source folder `{}` does not exists under template path",
                kind,
                kind,
            );
        }

        let destination = root_path.join(kind.to_string());
        if contrib::fs::is_dir_nonempty(&destination)? && !self.force {
            anyhow::bail!(
                "destination {} is not empty, use the `--force` flag to re-install them",
                destination.display()
            );
        }

        let mut options = rsync::SyncOptions::new();
        options.add_include("karimado")?;
        options.add_include("karimado/**")?;
        options.add_include(&format!("{}", kind))?;
        options.add_include(&format!("{}/**", kind))?;
        rsync::sync(&template_path, root_path, &options)?;

        log::info!("");
        Ok(())
    }
}

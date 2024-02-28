use anyhow::Result;
use clap::Args;
use std::path::Path;
use url::Url;

use crate::{cli::contrib, config};

#[derive(Args)]
pub(crate) struct InstallCommand {}

impl InstallCommand {
    pub(crate) fn execute(&self) -> Result<()> {
        let root_path = contrib::root_path()?;
        let config_file = root_path.join("karimado.toml");
        let config = config::from_config_file(&config_file)?;

        if let Some(web) = config.workspace.web {
            let scaffold = web.scaffold;
            let path = root_path.join("web");
            self.install_scaffold(&scaffold.url, &scaffold.template_path, &path)?;
        }

        if let Some(server) = config.workspace.server {
            let scaffold = server.scaffold;
            let path = root_path.join("server");
            self.install_scaffold(&scaffold.url, &scaffold.template_path, &path)?;
        }

        Ok(())
    }

    fn install_scaffold(&self, _url: &Url, _template_path: &str, to: &Path) -> Result<()> {
        if to.exists() && to.read_dir()?.next().is_some() {
            anyhow::bail!("destination {:?} is not empty", to)
        }

        Ok(())
    }
}

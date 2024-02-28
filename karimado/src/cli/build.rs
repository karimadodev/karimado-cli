use anyhow::Result;
use clap::Args;

use crate::{cli::contrib, config};

#[derive(Args)]
pub(crate) struct BuildCommand {
    /// Watch input files
    #[arg(long)]
    watch: bool,
}

impl BuildCommand {
    pub(crate) fn execute(&self) -> Result<()> {
        let root_path = contrib::root_path()?;
        let config_file = root_path.join("karimado.toml");
        let config = config::from_config_file(&config_file)?;

        if let Some(web) = config.workspace.web {
            println!("{:?}", web.name);
        }

        if let Some(server) = config.workspace.server {
            println!("{:?}", server.name);
        }

        Ok(())
    }
}

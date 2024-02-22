mod workspace;

use serde::Deserialize;

use crate::config::workspace::Workspace;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Config {
    workspace: Workspace,
}

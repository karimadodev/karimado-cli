use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Workspace {
    pub(crate) web: Option<Web>,
    pub(crate) server: Option<Server>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Web {
    pub(crate) name: String,
    pub(crate) scaffold: WebScaffold,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct WebScaffold {
    pub(crate) url: Url,
    #[serde(default)]
    pub(crate) template_path: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Server {
    pub(crate) name: String,
    pub(crate) scaffold: ServerScaffold,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ServerScaffold {
    pub(crate) url: Url,
    #[serde(default)]
    pub(crate) template_path: String,
}

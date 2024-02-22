use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Workspace {
    pub(crate) web: WebScaffold,
    pub(crate) server: ServerScaffold,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Web {
    pub(crate) name: String,
    pub(crate) scaffold: WebScaffold,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct WebScaffold {
    pub(crate) url: String,
    pub(crate) template_path: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Server {
    pub(crate) name: String,
    pub(crate) scaffold: ServerScaffold,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ServerScaffold {
    pub(crate) url: String,
    pub(crate) template_path: String,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("task `{0}` does not exist")]
    TaskNotFound(String),
    #[error("{0}")]
    TaskRunError(String),
    #[error(transparent)]
    TaskFileParseError(#[from] TaskFileParseErrorKind),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, thiserror::Error)]
pub enum TaskFileParseErrorKind {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    TomlError(#[from] toml::de::Error),
    #[error("{0}")]
    ParseIncludeError(String),
    #[error(transparent)]
    ParseTaskCommandError(#[from] handlebars::RenderError),
}

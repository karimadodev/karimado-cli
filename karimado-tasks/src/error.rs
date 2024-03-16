pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("task `{0}` does not exist")]
    TaskNotFound(String),
    #[error("{0}")]
    TaskRunFailed(String),
    #[error(transparent)]
    TaskFileParseFailed(#[from] TaskFileParseFailedKind),
}

#[derive(Debug, thiserror::Error)]
pub enum TaskFileParseFailedKind {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    TomlError(#[from] toml::de::Error),
    #[error("{0}")]
    ParseIncludeFailed(String),
}

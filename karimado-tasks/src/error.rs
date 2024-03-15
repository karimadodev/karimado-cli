pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("task `{0}` does not exist")]
    TaskNotFound(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

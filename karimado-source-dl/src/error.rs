pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    UrlParseError(#[from] UrlParseErrorKind),
    #[error(transparent)]
    SourceDownloadError(#[from] SourceDownloadErrorKind),
}

#[derive(Debug, thiserror::Error)]
pub enum UrlParseErrorKind {
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    #[error("{0}")]
    UnknownScheme(String),
}

#[derive(Debug, thiserror::Error)]
pub enum SourceDownloadErrorKind {
    #[error(transparent)]
    Git2Error(#[from] git2::Error),
}

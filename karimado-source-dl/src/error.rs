pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    UrlParseError(#[from] UrlParseErrorKind),
    #[error(transparent)]
    SourceDownloadError(#[from] SourceDownloadErrorKind),
    #[error(transparent)]
    SourceDecompressError(#[from] SourceDecompressErrorKind),
}

#[derive(Debug, thiserror::Error)]
pub enum UrlParseErrorKind {
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    #[error("{0}")]
    UnknownScheme(String),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, thiserror::Error)]
pub enum SourceDownloadErrorKind {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    Git2Error(#[from] git2::Error),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum SourceDecompressErrorKind {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ZipError(#[from] zip::result::ZipError),
    #[error("{0}")]
    UnknownMimeType(String),
}

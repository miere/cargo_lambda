use thiserror::Error as ThisError;

/// Default error handler
pub type Result<T> = std::result::Result<T, KnownFailures>;

/// Wraps commons failures that might happen in this project
#[derive(ThisError, Debug)]
pub enum KnownFailures {

    #[error("Failed to build or compile your software")]
    FailedToCompile,

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    ZipCompressionError(#[from] zip::result::ZipError),

    #[error(transparent)]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    TomlError(#[from] toml::de::Error),
}

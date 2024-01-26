pub type JfResult<T> = Result<T, JfError>;

#[derive(Debug, thiserror::Error)]
pub enum JfError {
    #[error("IO error occurred: {0}")]
    IoError(#[from] std::io::Error),
    #[error("SerdeJsonError occurred: {0}")]
    MpscRecvError(#[from] std::sync::mpsc::RecvError),
    #[error("GlobError occurred: {0}")]
    GlobError(#[from] glob::GlobError),
    #[error("Toml Deserialize error occurred: {0}")]
    TomlDeError(#[from] toml::de::Error),
    #[error("TokioJoinError occurred: {0}")]
    TokioJoinError(#[from] tokio::task::JoinError),
    #[error("TokioTryLockError occurred: {0}")]
    TokioTryLockError(#[from] tokio::sync::TryLockError),
    #[error("NotifyError occurred: {0}")]
    NotifyError(#[from] notify::Error),
    #[error("GlobPatternError occurred: {0}")]
    GlobPatternError(#[from] glob::PatternError),

    #[error("{0:?}")]
    Multi(Vec<JfError>),

    #[error("{0}")]
    Custom(String),
}

pub trait IntoJfError {
    fn into_jf_error(self) -> JfError;
}

impl<S: AsRef<str>> IntoJfError for S {
    fn into_jf_error(self) -> JfError {
        JfError::Custom(self.as_ref().to_string())
    }
}

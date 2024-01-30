pub type JfResult<T> = Result<T, JfError>;

#[derive(Debug, thiserror::Error)]
pub enum JfError {
    #[error("IO error occurred: {0}")]
    IoError(#[from] std::io::Error),
    #[error("std::sync::mpsc::RecvError occurred: {0}")]
    SyncMpscMpscRecvError(#[from] std::sync::mpsc::RecvError),
    #[error("std::sync::mpsc::RecvTimeoutError occurred: {0}")]
    SyncMpscRecvTimeoutError(#[from] std::sync::mpsc::RecvTimeoutError),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn cover() {
        let err = JfError::Custom("test".into());
        assert_eq!(err.to_string(), "test");
        println!("{:?}", err)
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn multi() {
        let err = JfError::Multi(vec![
            JfError::Custom("test1".into()),
            JfError::Custom("test2".into()),
        ]);

        assert_eq!(err.to_string(), "[Custom(\"test1\"), Custom(\"test2\")]");
    }
}

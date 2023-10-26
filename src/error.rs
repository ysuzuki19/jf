use thiserror::Error;

pub type CmdResult<T> = Result<T, CmdError>;

#[derive(Debug, Error)]
pub enum CmdError {
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

    #[error("Taskdef(name={0}) not found")]
    TaskdefNotFound(String),

    // #[error("Taskdef(name={0}) occurred parsing error\n{1}")]
    // TaskdefParse(String, String),
    #[error("Taskdef(mode={0}) require the field: \"{1}\"")]
    TaskdefMissingField(String, String),

    #[error("Undefined error occurred: {0}")]
    _Undefined(String),

    #[error("{0}")]
    Custom(String),
}

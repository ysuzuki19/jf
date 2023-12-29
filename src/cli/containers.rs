use std::path::PathBuf;

use crate::cli::LogLevel;

pub struct Context {
    pub log_level: LogLevel,
}

pub struct Options {
    pub cfg: Option<PathBuf>,
}

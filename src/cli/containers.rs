use std::path::PathBuf;

use crate::cli::LogLevel;

pub struct Ctx {
    pub log_level: LogLevel,
}

pub struct Opts {
    pub cfg: Option<PathBuf>,
}

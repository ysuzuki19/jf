pub mod action;
mod log_level;

use std::path::PathBuf;

pub use action::Action;

pub use self::log_level::LogLevel;

pub struct Ctx {
    pub log_level: LogLevel,
}

pub struct Opts {
    pub cfg: Option<PathBuf>,
}

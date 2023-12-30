pub mod action;
mod logger;

use std::path::PathBuf;

pub use action::Action;

pub use self::logger::{LogLevel, Logger};

pub struct Ctx {
    pub logger: Logger,
}

pub struct Opts {
    pub cfg: Option<PathBuf>,
}

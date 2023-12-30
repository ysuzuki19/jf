pub mod action;
mod logger;

use std::path::PathBuf;

pub use logger::{LogLevel, Logger};

pub struct Ctx {
    pub logger: Logger,
}

pub struct Opts {
    pub cfg: Option<PathBuf>,
}

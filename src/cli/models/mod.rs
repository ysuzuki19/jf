pub mod action;
mod logger;

use std::path::PathBuf;

pub use logger::{LogLevel, Logger};

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Ctx {
    pub logger: Logger,
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Opts {
    pub cfg: Option<PathBuf>,
}

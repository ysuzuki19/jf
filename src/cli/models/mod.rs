pub mod action;
mod logger;

use std::path::PathBuf;

pub use logger::{LogLevel, Logger};

#[cfg_attr(test, derive(Debug, PartialEq, Default))]
pub struct Ctx {
    pub logger: Logger,
}

#[cfg(test)]
impl crate::testutil::Fixture for Ctx {
    fn fixture() -> Self {
        Self {
            logger: crate::testutil::Fixture::fixture(),
        }
    }
}

#[cfg_attr(test, derive(Debug, PartialEq, Default))]
pub struct Opts {
    pub cfg: Option<PathBuf>,
}

#[cfg(test)]
impl crate::testutil::Fixture for Opts {
    fn fixture() -> Self {
        let cfg = PathBuf::from(".").join("tests").join("fixtures");
        Opts { cfg: Some(cfg) }
    }
}

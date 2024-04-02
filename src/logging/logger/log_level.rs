// SPDX-License-Identifier: MPL-2.0
use clap::ValueEnum;

#[derive(Clone, Default, Copy, PartialEq, PartialOrd, ValueEnum)]
#[cfg_attr(test, derive(Debug))]
pub enum LogLevel {
    /// No log output
    None,
    Error,
    Warn,
    #[default]
    Info,
    Debug,
}

impl LogLevel {
    pub fn short(&self) -> &str {
        match self {
            LogLevel::None => "N",
            LogLevel::Error => "E",
            LogLevel::Warn => "W",
            LogLevel::Info => "I",
            LogLevel::Debug => "D",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::testutil::*;

    impl Fixture for LogLevel {
        fn fixture() -> Self {
            Self::Debug
        }
    }

    #[test]
    fn log_level() {
        assert_eq!(LogLevel::None, LogLevel::from_str("none", false).unwrap());
        assert_eq!(LogLevel::Error, LogLevel::from_str("error", false).unwrap());
        assert_eq!(LogLevel::Warn, LogLevel::from_str("warn", false).unwrap());
        assert_eq!(LogLevel::Info, LogLevel::from_str("info", false).unwrap());
        assert_eq!(LogLevel::Debug, LogLevel::from_str("debug", false).unwrap());
        assert!(LogLevel::from_str("anything", false).is_err());
    }

    #[test]
    fn log_level_default() {
        assert_eq!(LogLevel::Info, LogLevel::default());
    }

    #[test]
    fn log_level_fixture() {
        assert_eq!(LogLevel::Debug, LogLevel::fixture());
    }
}

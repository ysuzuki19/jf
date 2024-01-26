use clap::ValueEnum;

#[derive(Clone, Default, Copy, PartialEq, PartialOrd, ValueEnum)]
#[cfg_attr(test, derive(Debug))]
#[non_exhaustive]
pub enum LogLevel {
    /// No log output
    None,
    Error,
    Warn,
    #[default]
    Info,
    Debug,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::*;

    impl Fixture for LogLevel {
        fn fixture() -> Self {
            Self::Debug
        }
    }

    #[test]
    fn test_log_level() {
        assert_eq!(LogLevel::None, LogLevel::from_str("none", false).unwrap());
        assert_eq!(LogLevel::Error, LogLevel::from_str("error", false).unwrap());
        assert_eq!(LogLevel::Warn, LogLevel::from_str("warn", false).unwrap());
        assert_eq!(LogLevel::Info, LogLevel::from_str("info", false).unwrap());
        assert_eq!(LogLevel::Debug, LogLevel::from_str("debug", false).unwrap());
        assert!(LogLevel::from_str("anything", false).is_err());
    }

    #[test]
    fn test_log_level_default() {
        assert_eq!(LogLevel::Info, LogLevel::default());
    }

    #[test]
    fn test_log_level_fixture() {
        assert_eq!(LogLevel::Debug, LogLevel::fixture());
    }
}

use clap::builder::PossibleValue;
use clap::ValueEnum;

#[derive(Clone, Default, Copy, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum LogLevel {
    /// No log output
    None,
    #[default]
    Error,
    Warn,
    Info,
}

impl ValueEnum for LogLevel {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            LogLevel::None,
            LogLevel::Error,
            LogLevel::Warn,
            LogLevel::Info,
        ]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            LogLevel::None => PossibleValue::new("none"),
            LogLevel::Error => PossibleValue::new("error"),
            LogLevel::Warn => PossibleValue::new("warn"),
            LogLevel::Info => PossibleValue::new("info"),
        })
    }
}

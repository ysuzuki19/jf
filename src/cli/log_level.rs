use clap::builder::PossibleValue;
use clap::ValueEnum;

#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub enum LogLevel {
    Info,
    Warn,
    #[default]
    Error,
    None,
}

impl ValueEnum for LogLevel {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            LogLevel::Info,
            LogLevel::Warn,
            LogLevel::Error,
            LogLevel::None,
        ]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            LogLevel::Info => PossibleValue::new("info"),
            LogLevel::Warn => PossibleValue::new("warn"),
            LogLevel::Error => PossibleValue::new("error"),
            LogLevel::None => PossibleValue::new("none"),
        })
    }
}

// SPDX-License-Identifier: MPL-2.0
use super::LogLevel;

pub(super) struct LogGenerator {
    log_level: LogLevel,
    message: String,
    with_time: bool,
}

impl LogGenerator {
    pub fn new(log_level: LogLevel, message: String) -> Self {
        Self {
            log_level,
            message,
            #[cfg(not(test))]
            with_time: true,
            #[cfg(test)]
            with_time: false,
        }
    }

    pub fn gen(self) -> String {
        if self.with_time {
            let now = chrono::Local::now().format("%H:%M:%S.%3f");
            format!("{}[{}] {}", now, self.log_level.short(), self.message)
        } else {
            format!("[{}] {}", self.log_level.short(), self.message)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::testutil::*;

    impl Fixture for LogGenerator {
        fn fixture() -> Self {
            Self::new(Fixture::fixture(), "test".to_string())
        }
    }

    #[test]
    fn log_generator() {
        let log_gen = LogGenerator::fixture();
        let text = log_gen.gen();
        assert_eq!(String::from("[D] test"), text);
    }
}

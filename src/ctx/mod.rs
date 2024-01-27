pub mod logger;

#[derive(Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Ctx<LR: logger::LogWriter> {
    pub logger: logger::Logger<LR>,
}

#[cfg(test)]
impl Ctx<logger::MockLogWriter> {
    pub fn new(log_level: logger::LogLevel) -> Self {
        Self {
            logger: logger::Logger::new(log_level),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::testutil::*;

    use super::*;

    impl Fixture for Ctx<MockLogWriter> {
        #[cfg_attr(coverage, coverage(off))]
        fn fixture() -> Self {
            Self {
                logger: Fixture::fixture(),
            }
        }
    }
}

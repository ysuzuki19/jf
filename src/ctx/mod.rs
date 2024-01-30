pub mod logger;

#[derive(Clone)]
#[cfg_attr(test, derive(PartialEq, Debug))]
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
    use crate::util::testutil::*;

    use super::*;

    impl Fixture for Ctx<MockLogWriter> {
        #[cfg_attr(coverage, coverage(off))]
        fn fixture() -> Self {
            Self {
                logger: Fixture::fixture(),
            }
        }
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn cover() {
        let ctx = Ctx::<MockLogWriter>::fixture();
        println!("{:?}", ctx); // Cover derive(Debug)
    }
}

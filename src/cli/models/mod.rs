pub mod action;

use std::path::PathBuf;

#[cfg_attr(test, derive(PartialEq, Default, Debug))]
pub struct Opts {
    pub cfg: Option<PathBuf>,
}

#[cfg(test)]
mod tests {
    use crate::util::testutil::*;

    use super::*;

    impl Fixture for Opts {
        #[cfg_attr(coverage, coverage(off))]
        fn fixture() -> Self {
            let cfg = PathBuf::from(".").join("tests").join("fixtures");
            Opts { cfg: Some(cfg) }
        }
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn cover() {
        println!("{:?}", Opts::fixture()); // Cover derive(Debug)
    }
}

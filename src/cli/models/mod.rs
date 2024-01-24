pub mod action;

use std::path::PathBuf;

#[cfg_attr(test, derive(PartialEq, Default))]
pub struct Opts {
    pub cfg: Option<PathBuf>,
}

#[cfg(test)]
mod tests {
    use crate::testutil::*;

    use super::*;

    impl Fixture for Opts {
        #[cfg_attr(coverage, coverage(off))]
        fn fixture() -> Self {
            let cfg = PathBuf::from(".").join("tests").join("fixtures");
            Opts { cfg: Some(cfg) }
        }
    }
}

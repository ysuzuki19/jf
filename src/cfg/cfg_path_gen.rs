use std::path::PathBuf;

pub struct CfgPathGen(Option<PathBuf>);

const DEFAULT_CFG_NAME: &str = "jf.toml";

/// cfg file path Generator
impl CfgPathGen {
    pub fn new(input: Option<PathBuf>) -> Self {
        Self(input)
    }

    /// parse & generate cfg file path
    pub fn gen(self) -> PathBuf {
        match self.0 {
            None => DEFAULT_CFG_NAME.into(),
            Some(input) => {
                if input.is_dir() {
                    input.join(DEFAULT_CFG_NAME)
                } else {
                    input
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn fixtures_dir() -> PathBuf {
        PathBuf::from(".").join("tests").join("fixtures")
    }

    #[test]
    fn test_gen_default() {
        let path = CfgPathGen::new(None).gen();
        assert_eq!(path, PathBuf::from(DEFAULT_CFG_NAME));
    }

    #[test]
    fn test_gen_from_dir() {
        let dir = fixtures_dir();
        let path = CfgPathGen::new(Some(dir.clone())).gen();
        assert_eq!(path, dir.join(DEFAULT_CFG_NAME));
    }

    #[test]
    fn test_gen_from_cfg() {
        let file_path = fixtures_dir().join(DEFAULT_CFG_NAME);
        let path = CfgPathGen::new(Some(file_path.clone())).gen();
        assert_eq!(path, file_path);
    }
}
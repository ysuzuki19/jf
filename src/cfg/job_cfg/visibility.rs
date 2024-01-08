use serde::Deserialize;

#[derive(Clone, Deserialize, Default)]
pub enum Visibility {
    #[serde(rename = "private")]
    Private,
    #[default]
    #[serde(rename = "public")]
    Public,
}

impl Visibility {
    #[cfg(test)]
    pub fn is_private(&self) -> bool {
        matches!(self, Visibility::Private)
    }

    pub fn is_public(&self) -> bool {
        matches!(self, Visibility::Public)
    }
}

#[cfg(test)]
mod tests {
    use crate::error::JfResult;

    use super::*;

    #[test]
    #[coverage(off)]
    fn deserialize_default() -> JfResult<()> {
        let visibility = Visibility::Public;
        assert!(visibility.is_public());
        assert!(!visibility.is_private());
        Ok(())
    }

    #[test]
    #[coverage(off)]
    fn deserialize_private() -> JfResult<()> {
        let visibility = Visibility::Private;
        assert!(visibility.is_private());
        assert!(!visibility.is_public());
        Ok(())
    }
}

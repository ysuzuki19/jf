pub enum JoinStatus {
    Succeed,
    Failed,
}

impl JoinStatus {
    #[cfg(test)]
    pub fn is_succeed(&self) -> bool {
        matches!(self, Self::Succeed)
    }

    pub fn is_failed(&self) -> bool {
        matches!(self, Self::Failed)
    }
}

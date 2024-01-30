#[derive(Clone)]
pub struct ReadOnly<T>(T);

impl<T> ReadOnly<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }

    pub fn read(&self) -> &T {
        &self.0
    }

    pub fn unwrap(self) -> T {
        self.0
    }
}

impl<T> From<T> for ReadOnly<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

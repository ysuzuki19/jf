// SPDX-License-Identifier: MPL-2.0
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

#[derive(Clone)]
pub struct Canceller {
    signal: Arc<AtomicBool>,
}

impl Canceller {
    pub fn new() -> Self {
        Self {
            signal: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn cancel(&self) {
        self.signal.store(true, Ordering::Relaxed);
    }

    pub fn is_canceled(&self) -> bool {
        self.signal.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let canceller = Canceller::new();
        assert!(!canceller.is_canceled());
        canceller.cancel();
        assert!(canceller.is_canceled());
    }
}

// SPDX-License-Identifier: MPL-2.0
use std::sync::{atomic::AtomicBool, Arc};

pub(super) struct FinishNotify {
    is_finished: AtomicBool,
    notify: tokio::sync::Notify,
}

impl FinishNotify {
    pub fn new_arc() -> std::sync::Arc<Self> {
        Arc::new(Self::new())
    }

    pub fn new() -> Self {
        Self {
            is_finished: AtomicBool::new(false),
            notify: tokio::sync::Notify::new(),
        }
    }

    pub fn is_finished(&self) -> bool {
        self.is_finished.load(std::sync::atomic::Ordering::Acquire)
    }

    pub fn notify(&self) {
        self.is_finished
            .store(true, std::sync::atomic::Ordering::Release);
        self.notify.notify_waiters();
    }

    pub async fn wait(&self) {
        while !self.is_finished.load(std::sync::atomic::Ordering::Acquire) {
            self.notify.notified().await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[coverage(off)]
    async fn test_finish_notify() {
        let notify = FinishNotify::new();
        assert!(!notify.is_finished());
        notify.notify();
        assert!(notify.is_finished());
        notify.notify();
        assert!(notify.is_finished());
    }

    #[tokio::test]
    #[coverage(off)]
    async fn test_finish_notify_wait() {
        let notify = FinishNotify::new_arc();
        let handle = tokio::spawn({
            let notify = notify.clone();
            async move {
                notify.wait().await;
            }
        });
        tokio::time::sleep(std::time::Duration::from_millis(100)).await; // wait for the task to start
        assert!(!notify.is_finished());
        notify.notify();
        handle.await.unwrap();
        assert!(notify.is_finished());
    }
}

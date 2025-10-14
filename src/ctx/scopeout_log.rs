// SPDX-License-Identifier: MPL-2.0
use crate::logging::{log_generator, LogLevel};

pub struct ScopeoutLog {
    msg: Option<String>,
}

impl ScopeoutLog {
    pub fn new<S: AsRef<str>>(msg: Option<S>) -> Self {
        Self {
            msg: msg.map(|m| m.as_ref().to_owned()),
        }
    }
}

impl Drop for ScopeoutLog {
    fn drop(&mut self) {
        if let Some(ref msg) = self.msg {
            let line = log_generator::LogGenerator::new(LogLevel::Debug, msg.to_owned()).gen();
            println!("{line}");
        }
    }
}

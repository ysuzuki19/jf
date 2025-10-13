// SPDX-License-Identifier: MPL-2.0
#[derive(Clone)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct AppStack {
    stack: Vec<String>,
    cached: String,
}

impl AppStack {
    pub fn new(name: &str) -> Self {
        Self {
            stack: vec![name.to_owned()],
            cached: name.to_string(),
        }
    }

    pub fn stacked(&self) -> &String {
        &self.cached
    }

    pub fn push<S: AsRef<str>>(&mut self, name: S) {
        self.stack.push(name.as_ref().to_owned());
        self.cached = format!("{}.{}", self.cached, name.as_ref());
    }

    // pub fn pop(&mut self) {
    //     self.stack.pop();
    //     self.cached = self.stack.join(".");
    // }
}

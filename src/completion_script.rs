pub struct CompletionScript(String);

impl CompletionScript {
    pub fn new() -> Self {
        Self(String::new())
    }

    pub fn apply_dynamic_completion_for_taskname(&mut self) {
        self.0 = self.0.replace("\"<TASK_NAME>\"", "$(cmd list)");
    }

    pub fn script(&self) -> String {
        self.0.clone()
    }
}

impl std::io::Write for CompletionScript {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let s = std::str::from_utf8(buf).unwrap();
        self.0.push_str(s);
        Ok(s.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}

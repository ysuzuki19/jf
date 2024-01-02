pub struct WritableString(String);

impl WritableString {
    pub fn new() -> Self {
        Self(String::new())
    }
}

impl ToString for WritableString {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl std::io::Write for WritableString {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let s = std::str::from_utf8(buf)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        self.0.push_str(s);
        Ok(s.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    #[test]
    fn new() {
        let buf = WritableString::new();
        assert_eq!(buf.to_string(), "");
    }

    #[test]
    fn write_flush() {
        let mut buf = WritableString::new();
        buf.write_all("test".as_bytes()).unwrap();
        assert_eq!(buf.to_string(), "test");
        buf.flush().unwrap();
        assert_eq!(buf.to_string(), "test");
    }
}
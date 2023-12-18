use super::args::Args;

struct WritableString(String);

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

pub fn generate<G>(shell: G) -> String
where
    G: clap_complete::Generator,
{
    let mut cmd = <Args as clap::CommandFactory>::command();
    let bin_name = cmd.get_name().to_owned();

    let mut buf = WritableString::new();

    clap_complete::generate(shell, &mut cmd, bin_name, &mut buf);

    // For bash dynamic completion
    buf.to_string().replace("\"<JOB_NAME>\"", "$(jf list)")
}

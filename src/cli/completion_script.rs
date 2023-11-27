use super::args::Args;

struct WritableString(String);

impl WritableString {
    pub fn new() -> Self {
        Self(String::new())
    }

    pub fn string(&self) -> String {
        self.0.clone()
    }
}

impl std::io::Write for WritableString {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let s = std::str::from_utf8(buf).unwrap();
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

    let mut out = WritableString::new();

    clap_complete::generate(shell, &mut cmd, bin_name, &mut out);

    out.string().replace("\"<TASK_NAME>\"", "$(cmd list)")
}

mod writable_string;

use super::args::Args;

const COMMAND_LIST_WITHOUT_LOG: &str = "$(jf --list --log-level none)";

pub fn generate(shell: clap_complete::Shell) -> String {
    let mut cmd = <Args as clap::CommandFactory>::command();
    let bin_name = cmd.get_name().to_owned();

    let script = {
        let mut buf = writable_string::WritableString::new();
        clap_complete::generate(shell, &mut cmd, bin_name, &mut buf);
        buf.to_string()
    };

    // Optimize completion script such as dynamic completion
    match shell {
        clap_complete::Shell::Bash => {
            script
                // For Ubuntu/bash
                .replace("\"<JOB_NAME>\"", COMMAND_LIST_WITHOUT_LOG)
                // For MacOS/bash
                .replace("[JOB_NAME]", COMMAND_LIST_WITHOUT_LOG)
        }
        _ => script,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_bash() {
        let script = generate(clap_complete::Shell::Bash);
        // check optimized
        assert!(!script.contains("JOB_NAME") && script.contains(COMMAND_LIST_WITHOUT_LOG));
    }

    #[test]
    fn generate_others() {
        let script = generate(clap_complete::Shell::Zsh);
        assert!(!script.contains(COMMAND_LIST_WITHOUT_LOG)); // check unoptimized
        let script = generate(clap_complete::Shell::Fish);
        assert!(!script.contains(COMMAND_LIST_WITHOUT_LOG)); // check unoptimized
        let script = generate(clap_complete::Shell::PowerShell);
        assert!(!script.contains(COMMAND_LIST_WITHOUT_LOG)); // check unoptimized
        let script = generate(clap_complete::Shell::Elvish);
        assert!(!script.contains(COMMAND_LIST_WITHOUT_LOG)); // check unoptimized
    }
}

mod cfg;
mod commander;
mod completion_script;
mod error;
mod task;
mod taskdef;

use clap::Parser;

use commander::Commander;
use error::CmdResult;

use crate::completion_script::CompletionScript;

#[derive(Parser, Debug)]
#[command(author = "ysuzuki19", version, about, long_about = None)]
#[command(disable_help_flag = true)]
struct Args {
    #[command(subcommand)]
    sub_command: Option<SubCommand>,

    #[arg(long)]
    verbose: bool,
}

#[derive(Parser, Debug, Clone)]
enum SubCommand {
    Completion {
        shell: clap_complete::Shell,
    },
    #[command(about = "Run a task")]
    Run {
        task_name: String,
    },
    #[command(about = "Description a task")]
    Description {
        task_name: String,
    },
    #[command(about = "List tasks")]
    List,
}

impl SubCommand {
    pub async fn process(self, commander: Commander) -> CmdResult<Option<String>> {
        match self {
            SubCommand::Completion { shell } => {
                let mut cmd = <Args as clap::CommandFactory>::command();
                let bin_name = cmd.get_name().to_owned();
                let mut completion_script = CompletionScript::new();

                clap_complete::generate(shell, &mut cmd, bin_name, &mut completion_script);

                completion_script.apply_dynamic_completion_for_taskname();

                Ok(Some(completion_script.script()))
            }
            SubCommand::Run { task_name } => {
                commander.run(task_name).await?;
                Ok(None)
            }
            SubCommand::Description { task_name } => Ok(Some(commander.description(task_name)?)),
            SubCommand::List => Ok(Some(commander.list().join("\n"))),
        }
    }
}

async fn cli() -> CmdResult<()> {
    let cfg = cfg::Cfg::load()?;
    let commander = commander::Commander::new(cfg)?;

    let args = Args::parse();
    if let Some(sub_command) = args.sub_command {
        if let Some(output) = sub_command.process(commander).await? {
            println!("{}", output);
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = cli().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

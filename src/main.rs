mod cfg;
mod commander;
mod completion_script;
mod error;
mod task;
mod taskdef;

use clap::Parser;

use commander::Commander;
use error::CmdResult;

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
    #[command(about = "Generate completion script")]
    Completion { shell: clap_complete::Shell },
    #[command(about = "Run a task")]
    Run { task_name: String },
    #[command(about = "Description a task")]
    Description { task_name: String },
    #[command(about = "List tasks")]
    List,
}

struct Cli {
    args: Args,
    commander: Commander,
}

impl Cli {
    pub fn load() -> CmdResult<Self> {
        let args = Args::parse();
        let cfg = cfg::Cfg::load()?;
        let commander = commander::Commander::new(cfg)?;
        Ok(Self { args, commander })
    }

    pub async fn run(self) -> CmdResult<()> {
        if let Some(sub_command) = self.args.sub_command {
            match sub_command {
                SubCommand::Completion { shell } => {
                    println!("{}", completion_script::generate(shell))
                }
                SubCommand::Run { task_name } => {
                    self.commander.run(task_name).await?;
                }
                SubCommand::Description { task_name } => {
                    println!("{}", self.commander.description(task_name)?);
                }
                SubCommand::List => {
                    println!("{}", self.commander.list().join("\n"));
                }
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    match Cli::load() {
        Ok(cli) => {
            if let Err(e) = cli.run().await {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Invalid Args or Config: {}", e);
            std::process::exit(1);
        }
    }
}

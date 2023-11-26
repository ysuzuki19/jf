mod cfg;
mod commander;
mod completion_script;
mod error;
mod task;
mod taskdef;

use clap::Parser;

use error::CmdResult;

use crate::completion_script::CompletionScript;

#[derive(Parser, Debug)]
#[command(author = "ysuzuki19", version, about, long_about = None)]
#[command(disable_help_flag = true)]
struct Args {
    #[command(subcommand)]
    sub_command: Option<SubCommand>,
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

async fn cli(args: Args) -> CmdResult<()> {
    let cfg_file = "cmdrc.toml";
    let cfg_contents = std::fs::read_to_string(cfg_file)?;
    let cmd_cfg: cfg::CmdCfg = toml::from_str(&cfg_contents)?;

    let commander = commander::Commander::new(cmd_cfg)?;
    // // let task_name = "incl10-parallel-watch".to_string();
    // let task_name = "sequential-echos-watch".to_string();
    // commander.description(task_name.clone())?;
    // commander.run(task_name).await?;

    if let Some(sub_command) = args.sub_command {
        match sub_command {
            SubCommand::Completion { shell } => {
                let mut cmd = <Args as clap::CommandFactory>::command();
                let bin_name = cmd.get_name().to_owned();
                let mut completion_script = CompletionScript::new();

                clap_complete::generate(shell, &mut cmd, bin_name, &mut completion_script);

                completion_script.apply_dynamic_completion_for_taskname();

                println!("{}", completion_script.script());
            }
            SubCommand::Run { task_name } => {
                commander.run(task_name).await?;
            }
            SubCommand::Description { task_name } => {
                println!("{}", commander.description(task_name)?);
            }
            SubCommand::List => {
                for task_name in commander.list() {
                    println!("{}", task_name);
                }
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match cli(args).await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

mod cfg;
mod commander;
mod error;
mod task;
mod taskdef;

use clap::Parser;

use error::CmdResult;

#[derive(Parser, Debug)]
#[command(author = "ysuzuki19", version, about, long_about = None)]
#[command(disable_help_flag = true)]
struct Args {
    #[command(subcommand)]
    sub_command: Option<SubCommand>,
}

#[derive(Parser, Debug, Clone)]
enum SubCommand {
    // Completion {
    //     shell: Shell,
    // },
    #[command(about = "Run a task")]
    Run { task_name: String },

    #[command(about = "Description a task")]
    Description { task_name: String },
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
            // SubCommand::Completion { shell } => {
            //     let mut cmd = Args::command();
            //     let bin_name = cmd.get_name().to_owned();
            //     clap_complete::generate(shell, &mut cmd, bin_name, &mut std::io::stdout());
            // }
            SubCommand::Run { task_name } => {
                commander.run(task_name).await?;
            }
            SubCommand::Description { task_name } => {
                println!("{}", commander.description(task_name)?);
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

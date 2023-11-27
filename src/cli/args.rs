use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "ysuzuki19", version, about, long_about = None)]
#[command(disable_help_flag = true)]
pub struct Args {
    #[command(subcommand)]
    pub sub_command: Option<SubCommand>,

    #[arg(long)]
    verbose: bool,
}

#[derive(Parser, Debug, Clone)]
pub enum SubCommand {
    #[command(about = "Generate completion script")]
    Completion { shell: clap_complete::Shell },
    #[command(about = "Run a task")]
    Run { task_name: String },
    #[command(about = "Description a task")]
    Description { task_name: String },
    #[command(about = "List tasks")]
    List,
}

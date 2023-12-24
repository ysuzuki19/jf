use clap::Parser;

const AUTHOR: &str = "ysuzuki19";

#[derive(Parser, Debug, Clone)]
#[command(
    author = AUTHOR,
    version,
    about,
    long_about = None,
    disable_help_flag = true,
)]
pub struct Args {
    #[arg(long)]
    pub(super) help: bool,

    #[arg(long)]
    pub(super) verbose: bool,

    #[arg(long)]
    pub(super) validate: bool,

    #[arg(short, long)]
    pub(super) cfg: Option<String>,

    #[arg(long)]
    pub(super) completion: Option<clap_complete::Shell>,

    #[arg(long)]
    pub(super) list: bool,

    #[arg(long)]
    pub(super) description: bool,

    #[command()]
    pub(super) job_name: Option<String>,
}

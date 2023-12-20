use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(
    author = "ysuzuki19",
    version,
    about,
    long_about = None,
    disable_help_flag = true,
)]
pub struct Args {
    #[arg(long)]
    pub(super) verbose: bool,

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

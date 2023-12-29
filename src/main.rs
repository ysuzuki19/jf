mod cfg;
mod cli;
mod error;
mod job;
mod jobdef;

use crate::cli::LogLevel;

#[tokio::main]
async fn main() {
    match cli::Cli::load() {
        Ok(cli) => {
            let log_level = cli.context().log_level.clone();
            if let Err(e) = cli.run().await {
                match log_level {
                    LogLevel::None => {}
                    _ => eprintln!("{e}"),
                }
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Invalid Args or Config: {}", e);
            std::process::exit(1);
        }
    }
}

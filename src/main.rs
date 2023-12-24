mod cfg;
mod cli;
mod error;
mod job;
mod jobdef;

use std::sync::Arc;

use lazy_static::lazy_static;
use tokio::sync::RwLock;

use crate::cli::LogLevel;

lazy_static! {
    static ref LOG_LEVEL: Arc<RwLock<LogLevel>> = Arc::new(RwLock::new(LogLevel::default()));
}

#[tokio::main]
async fn main() {
    match cli::Cli::load() {
        Ok(cli) => {
            if let Err(e) = cli.run().await {
                match *LOG_LEVEL.read().await {
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

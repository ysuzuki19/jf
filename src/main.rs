#![cfg_attr(coverage, feature(coverage_attribute))]
mod cfg;
mod cli;
mod ctx;
mod job;
mod jobdef;
mod log_worker;
mod util;

use clap::Parser;
use log_worker::JfStdout;

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();
    let mut log_worker = log_worker::LogWorker::new();
    {
        let mut logger = log_worker.start(JfStdout::new(), args.log_level()).await;
        match cli::Cli::load(logger.clone(), args) {
            Ok(cli) => {
                if let Err(e) = cli.run().await {
                    let _ = logger.error(e.to_string()).await;
                    std::process::exit(1);
                }
            }
            Err(e) => {
                eprintln!("Failed to start: {}", e);
                std::process::exit(1);
            }
        }
    }
    log_worker.join().await.unwrap();
}

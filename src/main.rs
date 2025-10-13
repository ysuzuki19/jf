// SPDX-License-Identifier: MPL-2.0
#![feature(coverage_attribute)]
mod cfg;
mod cli;
mod ctx;
mod job;
mod jobdef;
mod logging;
mod util;

use clap::Parser;
use logging::Stdout;

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();
    let mut log_worker = logging::Worker::new();
    {
        let logger = log_worker.start(Stdout::new(), args.log_level()).await;
        match cli::Cli::load(logger.clone(), args) {
            Ok(cli) => {
                if let Err(e) = cli.run().await {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            }
            Err(e) => {
                eprintln!("Failed to start: {e}");
                std::process::exit(1);
            }
        }
    }
    log_worker.join().await.unwrap();
}

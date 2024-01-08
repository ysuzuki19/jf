#![cfg_attr(coverage, feature(coverage_attribute))]
mod cfg;
mod cli;
mod error;
mod job;
mod jobdef;
#[cfg(test)]
mod testutil;

use clap::Parser;

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();
    match cli::Cli::load(args) {
        Ok(cli) => {
            let logger = cli.ctx().logger.clone();
            if let Err(e) = cli.run().await {
                logger.error(e.to_string());
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Failed to start: {}", e);
            std::process::exit(1);
        }
    }
}

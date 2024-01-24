#![cfg_attr(coverage, feature(coverage_attribute))]
mod cfg;
mod cli;
mod ctx;
mod error;
mod job;
mod jobdef;
#[cfg(test)]
mod testutil;

use clap::Parser;

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();
    match cli::Cli::<ctx::logger::JfStdout>::load(args) {
        Ok(cli) => {
            let mut logger = cli.ctx().logger.clone();
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

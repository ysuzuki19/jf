mod cfg;
mod cli;
mod error;
mod job;
mod jobdef;

#[tokio::main]
async fn main() {
    match cli::Cli::load() {
        Ok(cli) => {
            let logger = cli.ctx().logger.clone();
            if let Err(e) = cli.run().await {
                logger.error(e.to_string());
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Invalid Args or Config: {}", e);
            std::process::exit(1);
        }
    }
}

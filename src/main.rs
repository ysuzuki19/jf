mod cfg;
mod cli;
mod error;
mod job;
mod jobdef;

#[tokio::main]
async fn main() {
    match cli::Cli::load() {
        Ok(cli) => {
            if let Err(e) = cli.run().await {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Invalid Args or Config: {}", e);
            std::process::exit(1);
        }
    }
}

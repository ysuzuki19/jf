mod cfg;
mod cli;
mod error;
mod job;
mod jobdef;

#[tokio::main]
async fn main() {
    match cli::Cli::load() {
        Ok(cli) => {
            let error_log_enabled = cli.error_log_enabled();
            if let Err(e) = cli.run().await {
                if error_log_enabled {
                    eprintln!("{}", e);
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

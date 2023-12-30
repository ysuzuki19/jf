mod cfg;
mod cli;
mod error;
mod job;
mod jobdef;

#[tokio::main]
async fn main() {
    match cli::Cli::load() {
        Ok(cli) => {
            let log_level = cli.ctx().log_level;
            if let Err(e) = cli.run().await {
                match log_level {
                    cli::LogLevel::None => {}
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

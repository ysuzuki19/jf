mod commander;
mod common;
mod config;
mod error;
mod task;
mod taskdef;

use error::CmdResult;

async fn cli() -> CmdResult<()> {
    let config_file = "cmdrc.toml";
    let config_contents = std::fs::read_to_string(config_file)?;
    let config: config::CmdConfig = toml::from_str(&config_contents)?;

    let commander = commander::Commander::new(config)?;
    let task_name = "incl10-parallel-watch".to_string();
    commander.description(task_name.clone())?;
    commander.run(task_name).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    match cli().await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

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
    println!("greet: {}", commander.description("greet".into())?);
    // commander.run("sleep".into()).await?;
    // commander.run("echo_hello".into()).await?;
    // commander.run("greet".into()).await?;
    // commander.run("greet-slow".into()).await?;
    // commander.run("greet-parallel".into()).await?;
    // commander.run("greet-watch").await?;
    // commander.run("incrementing".into()).await?;
    // commander.run("incrementing-watch".into()).await?;
    // commander.run("sequential-echos-watch".into()).await?;
    // commander.run("incrementing-sequential".into()).await?;
    // commander
    //     .run("incrementing-sequential-watch".into())
    //     .await?;
    // commander.run("run".into()).await?;
    commander.run("live-run".into()).await?;

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

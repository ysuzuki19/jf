use error::CmdResult;

mod commander;
mod config;
mod error;
mod task;
mod taskdef;

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
    commander.run("incrementing-watch".into()).await?;

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

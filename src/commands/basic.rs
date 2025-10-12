use tracing::info;

/// Execute the echo command
pub fn echo(text: &str) -> anyhow::Result<()> {
    info!(%text, "echoing text");
    println!("{}", text);
    Ok(())
}

/// Execute the ping command
pub fn ping() -> anyhow::Result<()> {
    println!("pong");
    Ok(())
}
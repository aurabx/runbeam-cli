use tracing::info;

pub fn harmony(text: &str) -> anyhow::Result<()> {
    info!(%text, "harmony");
    println!("{}", text);
    Ok(())
}

pub fn harmony_add(text: &str) -> anyhow::Result<()> {
    info!(%text, "harmony");
    println!("{}", text);
    Ok(())
}
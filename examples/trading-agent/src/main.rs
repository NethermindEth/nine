use anyhow::Result;
use config_toml::ConfigToml;
use n9_node::Node;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::try_init()?;
    Node::bootstrap().await?;
    Node::add(ConfigToml::new())?;

    signal::ctrl_c().await?;
    Node::shutdown().await?;
    Ok(())
}

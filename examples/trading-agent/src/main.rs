use anyhow::Result;
use n9_node::Node;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::try_init()?;
    Node::bootstrap().await?;
    signal::ctrl_c().await?;
    Node::shutdown().await?;
    Ok(())
}

use anyhow::Result;
use crb::agent::Runnable;
use n9_node::connector::{Connector, Key};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::try_init()?;
    let key = Key::generate();
    let connector = Connector::new(key);
    connector.run().await;
    Ok(())
}

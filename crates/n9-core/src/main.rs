use n9::server::EthereumService;
use rmcp::{
    transport::stdio,
    ServiceExt,
};

use log::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = EthereumService {}.serve(stdio()).await.inspect_err(|e| {
        error!("Error: {:?}", e);
    })?;

    info!("Server started, waiting for requests...");
    let quit_reason = service.waiting().await?;
    info!("Server quit: {:?}", quit_reason);
    Ok(())
}

use n9::server::EthereumNodeRequest;
use rmcp::{
    transport::stdio,
    ServiceExt,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = EthereumNodeRequest {
        node_url: String::new(),
    }.serve(stdio()).await.inspect_err(|e| {
            println!("Error: {:?}", e);
        })?;

    println!("Server started, waiting for requests...");
    let quit_reason = service.waiting().await?;
    println!("Server quit: {:?}", quit_reason);
    Ok(())
}

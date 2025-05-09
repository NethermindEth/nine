use ethers::providers::{Http, Middleware, Provider};
use rmcp::{
    ServerHandler,
    model::ServerInfo,
    schemars, tool,
};      

#[derive(Debug, Clone, serde::Deserialize, schemars::JsonSchema)]
pub struct EthereumNodeRequest {
    #[schemars(
        description = "Ethereum node URL (e.g., https://mainnet.infura.io/v3/YOUR_API_KEY)"
    )]
    pub node_url: String,
}
#[derive(Debug, Clone)]
pub struct EthereumService;


#[tool(tool_box)]
impl EthereumService {
    #[tool(description = "Get the latest Ethereum block information")]
    async fn get_latest_block(&self, #[tool(aggr)] request: EthereumNodeRequest) -> String {
        // Create a provider
        let provider = match Provider::<Http>::try_from(request.node_url) {
            Ok(p) => p,
            Err(e) => return format!("Error connecting to Ethereum node: {}", e),
        };

        // Get the latest block
        match provider.get_block(ethers::types::BlockNumber::Latest).await {
            Ok(Some(block)) => {
                let number = block.number.unwrap_or_default();
                let timestamp = block.timestamp;
                let hash = block.hash.unwrap_or_default();

                format!(
                    "Latest block: Number = {}, Time = {} ({}), Hash = {}",
                    number,
                    timestamp,
                    chrono::DateTime::from_timestamp(timestamp.as_u64() as i64, 0)
                        .map(|dt| dt.to_string())
                        .unwrap_or_else(|| "invalid timestamp".to_string()),
                    hash
                )
            }
            Ok(None) => format!("No block found"),
            Err(e) => format!("Error fetching latest block: {}", e),
        }
    }
}

// impl call_tool and list_tool by querying static toolbox
#[tool(tool_box)]
impl ServerHandler for EthereumService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("A simple server request for getting blockchain data".into()),
            ..Default::default()
        }
    }
}

use ethers::providers::{Http, Middleware, Provider};
use ethers::types::{Address, BlockId, H256};
use ethers::utils;
use rmcp::{ServerHandler, model::ServerInfo, schemars, tool};
use std::str::FromStr;

#[derive(Debug, Clone, serde::Deserialize, schemars::JsonSchema)]
pub struct EthereumNodeRequest {
    #[schemars(
        description = "Ethereum node URL (e.g., https://mainnet.infura.io/v3/YOUR_API_KEY)"
    )]
    pub node_url: String,
    #[schemars(description = "Ethereum block number")]
    pub block_number: Option<u64>,
    #[schemars(description = "Ethereum transaction hash")]
    pub transaction_hash: String,
    #[schemars(description = "Ethereum account address")]
    pub account_address: String,
}
#[derive(Debug, Clone)]
pub struct EthereumService;

#[tool(tool_box)]
impl EthereumService {
    #[tool(description = "Get the latest Ethereum block information")]
    async fn get_latest_block(&self, #[tool(aggr)] request: EthereumNodeRequest) -> String {
        let provider = match Provider::<Http>::try_from(request.node_url) {
            Ok(p) => p,
            Err(e) => return format!("Error connecting to Ethereum node: {}", e),
        };

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

    #[tool(description = "Get the Ethereum block information by block number")]
    async fn get_block(&self, #[tool(aggr)] request: EthereumNodeRequest) -> String {
        let provider = match Provider::<Http>::try_from(request.node_url) {
            Ok(p) => p,
            Err(e) => return format!("Error connecting to Ethereum node: {}", e),
        };

        match provider.get_block(request.block_number.unwrap()).await {
            Ok(Some(block)) => {
                let number = block.number.unwrap_or_default();
                let timestamp = block.timestamp;
                let hash = block.hash.unwrap_or_default();
                format!(
                    "Block: Number = {}, Time = {} ({}), Hash = {}",
                    number,
                    timestamp,
                    chrono::DateTime::from_timestamp(timestamp.as_u64() as i64, 0)
                        .map(|dt| dt.to_string())
                        .unwrap_or_else(|| "invalid timestamp".to_string()),
                    hash
                )
            }
            Ok(None) => format!("No block found"),
            Err(e) => format!("Error fetching block: {}", e),
        }
    }

    #[tool(description = "Get the Ethereum transaction information by transaction hash")]
    async fn get_transaction(&self, #[tool(aggr)] request: EthereumNodeRequest) -> String {
        let provider = match Provider::<Http>::try_from(request.node_url) {
            Ok(p) => p,
            Err(e) => return format!("Error connecting to Ethereum node: {}", e),
        };

        let tx_hash = match H256::from_str(&request.transaction_hash) {
            Ok(hash) => hash,
            Err(e) => {
                return format!(
                    "Invalid transaction hash format '{}': {}",
                    request.transaction_hash, e
                );
            }
        };

        match provider.get_transaction(tx_hash).await {
            Ok(Some(transaction)) => {
                let hash = transaction.hash;
                let from = transaction.from;
                let to = transaction.to.unwrap_or_default();
                let value = transaction.value;
                let block_number = transaction
                    .block_number
                    .map_or_else(|| "Pending".to_string(), |n| n.to_string());
                let gas_price = transaction
                    .gas_price
                    .map_or_else(|| "Pending".to_string(), |n| n.to_string());
                let gas_limit = transaction.gas.to_string();
                format!(
                    "Transaction: Hash = {}, From = {}, To = {}, Value = {}, Block = {}, Gas Price = {}, Gas Limit = {}",
                    hash, from, to, value, block_number, gas_price, gas_limit
                )
            }
            Ok(None) => format!("Transaction not found: {}", request.transaction_hash),
            Err(e) => format!(
                "Error fetching transaction {}: {}",
                request.transaction_hash, e
            ),
        }
    }

    #[tool(description = "Get all transactions in a given Ethereum block")]
    async fn get_block_transactions(&self, #[tool(aggr)] request: EthereumNodeRequest) -> String {
        let provider = match Provider::<Http>::try_from(request.node_url) {
            Ok(p) => p,
            Err(e) => return format!("Error connecting to Ethereum node: {}", e),
        };

        let block_with_txs = match provider
            .get_block_with_txs(request.block_number.unwrap())
            .await
        {
            Ok(Some(block)) => block,
            Ok(None) => return format!("Block not found: {}", request.block_number.unwrap()),
            Err(e) => return format!("Error fetching block with transactions: {}", e),
        };

        if block_with_txs.transactions.is_empty() {
            return format!(
                "No transactions found in block {}",
                request.block_number.unwrap()
            );
        }

        let transactions_iterator = block_with_txs.transactions.iter().take(10);

        let transactions_str = transactions_iterator
            .map(|tx| {
                let hash = tx.hash.to_string();
                let from = tx.from.to_string();
                let to = tx.to.map_or_else(
                    || "None (Contract Creation)".to_string(),
                    |addr| addr.to_string(),
                );
                let value = tx.value.to_string();
                let gas_price_str = tx
                    .gas_price
                    .map_or_else(|| "N/A".to_string(), |gp| gp.to_string());
                let gas_limit_str = tx.gas.to_string();

                format!(
                    "  Transaction: Hash: {}, From: {}, To: {}, Value: {}, Gas Price: {}, Gas Limit: {}",
                    hash, from, to, value, gas_price_str, gas_limit_str
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        format!(
            "Transactions in Block {}: {}",
            request.block_number.unwrap(),
            transactions_str
        )
    }

    #[tool(
        description = "Get the ETH balance of an account, optionally at a specific block number (defaults to latest)."
    )]
    async fn get_account_balance(&self, #[tool(aggr)] request: EthereumNodeRequest) -> String {
        let provider = match Provider::<Http>::try_from(request.node_url) {
            Ok(p) => p,
            Err(e) => return format!("Error connecting to Ethereum node: {}", e),
        };

        let address = match Address::from_str(&request.account_address) {
            Ok(addr) => addr,
            Err(e) => {
                return format!(
                    "Invalid account address format '{}': {}",
                    request.account_address, e
                );
            }
        };

        // Determine the block to query: specific number or latest (None)
        let block_id: Option<BlockId> = request
            .block_number
            .map(|bn_u64| ethers::types::BlockNumber::Number(bn_u64.into()).into());

        match provider.get_balance(address, block_id).await {
            Ok(balance_wei) => {
                // The balance is returned in Wei. Convert to Ether for readability.
                let balance_eth = utils::format_ether(balance_wei);
                let block_description = match request.block_number {
                    Some(bn) => format!("block {}", bn),
                    None => "latest block".to_string(),
                };
                format!(
                    "Balance of {}: {} ETH (at {})",
                    request.account_address,
                    balance_eth,
                    block_description // Use the correctly determined block_description
                )
            }
            Err(e) => format!(
                "Error fetching account balance for {}: {}",
                request.account_address, e
            ),
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

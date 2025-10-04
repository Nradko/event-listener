use std::env;
use std::time::Duration;

use alloy::{
    primitives::{address, Address},
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::{BlockNumberOrTag, Filter},
    sol,
};
use eyre::Result;
use futures_util::stream::StreamExt;
use serde_json::json;

const TOKEN_ADDRESS: Address = address!("0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238");
const SEPOLIA_CHAIN_ID: u64 = 11155111;

sol! {
    #[allow(missing_docs)]
    event Transfer(address indexed from, address indexed to, uint256 value);
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok(); // .ok() makes it not fail if .env file doesn't exist

    let rpc_url = env::var("RPC_URL").unwrap_or_else(|_| {
        println!("RPC_URL environment variable not set");
        panic!("No RPC_URL env variable");
    });

    let ws = WsConnect::new(rpc_url)
        .with_max_retries(30)
        .with_retry_interval(Duration::from_secs(10));

    let provider = ProviderBuilder::new().connect_ws(ws).await?;

    let chain_id = provider.get_chain_id().await?;
    if chain_id != SEPOLIA_CHAIN_ID {
        panic!(
            "Connected to wrong network. Expected Sepolia (11155111), got {}",
            chain_id
        );
    }

    let filter = Filter::new()
        .address(TOKEN_ADDRESS)
        .event("Transfer(address,address,uint256)")
        .from_block(BlockNumberOrTag::Latest);

    let sub = provider.subscribe_logs(&filter).await?;
    let mut stream = sub.into_stream();

    println!(
        "Listening for Transfer events on {} contract...",
        TOKEN_ADDRESS
    );

    while let Some(log) = stream.next().await {
        // Try to decode the log as a Transfer event
        match log.log_decode::<Transfer>() {
            Ok(decoded_log) => {
                let transfer_data = &decoded_log.data();

                // Create a structured JSON representation of the Transfer event
                let transfer_event = json!({
                    "from": transfer_data.from.to_string(),
                    "to": transfer_data.to.to_string(),
                    "value": transfer_data.value.to_string(),
                });

                println!("{}", serde_json::to_string_pretty(&transfer_event)?);
            }
            Err(e) => {
                println!("Failed to decode log as Transfer event: {e}");
                println!("Raw log: {log:?}");
            }
        }
    }

    Ok(())
}

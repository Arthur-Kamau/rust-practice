
use ethers::{
    prelude::*,
    providers::{Http, Provider},
};
use std::sync::Arc;
use std::str::FromStr;
use dotenv::dotenv;
use std::env;
abigen!(
    Counter,
    r#"[
        function getCount() public view returns (uint256)
        function increment() public
        function decrement() public
        event CounterIncremented(uint256 newCount)
        event CounterDecremented(uint256 newCount)
    ]"#
);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenv().ok();

    let ganache_url = env::var("GANACHE_URL").expect("GANACHE_URL not set");
    let mut private_key_1 = env::var("PRIVATE_KEY_1").expect("PRIVATE_KEY_1 not set");
    let mut private_key_2 = env::var("PRIVATE_KEY_2").expect("PRIVATE_KEY_2 not set");

    // Remove "0x" prefix if present
    if private_key_1.starts_with("0x") {
        private_key_1 = private_key_1.trim_start_matches("0x").to_string();
    }
    if private_key_2.starts_with("0x") {
        private_key_2 = private_key_2.trim_start_matches("0x").to_string();
    }


    // Connect to the network
    let provider = Provider::<Http>::try_from(ganache_url)?;
    let wallet = private_key_1.parse::<LocalWallet>()?.with_chain_id(1337u64);
    let client = Arc::new(SignerMiddleware::new(provider, wallet));


    // Contract address
    let address = "0x1b663112537Bbe963CA45a6e08fFF4f898Ae8833".parse::<Address>()?;

    // Create an instance of the contract
    let contract = Counter::new(address, client.clone());

    // Get the current count
    let count = contract.get_count().call().await?;
    println!("Current count: {}", count);

    // Increment the count
    let tx = contract.increment();
    let pending_tx = tx.send().await?;
    let receipt = pending_tx.await?;
    println!("Increment transaction receipt: {:?}", receipt);

    // Get the new count
    let new_count = contract.get_count().call().await?;
    println!("New count: {}", new_count);

    Ok(())
}
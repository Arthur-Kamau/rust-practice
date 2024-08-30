use ethers::prelude::*;
use ethers::providers::{Provider, Http};
use std::convert::TryFrom;
use std::env;
use dotenv::dotenv;
use std::sync::Arc;
use ethers::utils::format_units;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Load environment variables
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

    // Connect to the Ganache provider
    let provider = Provider::<Http>::try_from(ganache_url.as_str())?;
    let provider = Arc::new(provider);

    // Create wallets
    let wallet_1 = private_key_1.parse::<LocalWallet>()?.with_chain_id(1337u64);
    let wallet_2 = private_key_2.parse::<LocalWallet>()?.with_chain_id(1337u64);

    // Get the address of wallet_2
    let wallet_1_address = wallet_1.address();
    let wallet_1_balance =provider.get_balance(wallet_1_address, None).await? ;
    let wallet_1_balance_formatted =  format_units(wallet_1_balance, "ether")?;
    println!(
        "wallet_1_address => Balance for address {}   \n => {}",
        wallet_1_address,wallet_1_balance_formatted
    );

    // Get the address of wallet_2
    let wallet_2_address = wallet_2.address();

    let wallet_2_balance = provider.get_balance(wallet_2_address, None).await?;
    let wallet_2_balance_formatted =  format_units(wallet_1_balance, "ether")?;
    println!(
        "wallet_2_address => Balance for address {}: {}",
        wallet_2_address, wallet_2_balance_formatted
    );

    // Create a signer middleware
    let client = SignerMiddleware::new(provider.clone(), wallet_1);

    // Transfer 1 Ether from wallet_1 to wallet_2
    let tx = TransactionRequest::pay(wallet_2_address, ethers::utils::parse_ether(1.0)?);
    let pending_tx = client.send_transaction(tx, None).await?;

    // Wait for the transaction to be mined
    let receipt = pending_tx.await?;
    println!("Transaction mined: {:?}", receipt);



    println!(
        "Final wallet  Balance of wallet_1_address  {} \n \n wallet_2_address  {}",
        format_units(provider.get_balance(wallet_1_address, None).await?,"ether")?,
        format_units( provider.get_balance(wallet_2_address, None).await?,"ether")?
    );
    Ok(())
}

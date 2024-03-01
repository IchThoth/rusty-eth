use std::time::Duration;

use ethers::{
    prelude::{Address, LocalWallet, Middleware, Provider, Signer, TransactionRequest, U256},
    utils::Ganache,
};

use eyre::{ContextCompat, Result};
use hex::ToHex;

#[tokio::main]

wallet()
//create a dev wallet
async fn wallet() -> Result<()> {
    let mnemonic = "";
    let ganache = Ganache::new().mnemonic(mnemonic).spawn();
    println!("HTTP Endpoint: {}",ganache.endpoint());

    let user_wallet: LocalWallet = ganache.keys()[0].clone().into();
    let wallet_first_address = user_wallet.address();

    println!(
        "wallet first address: {}",
         wallet_first_address.encode_hex::<String>()
    );
}

fn main() {
    
}

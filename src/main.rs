mod account;
mod config;
mod erc4337;
mod execution;
use crate::account::{BaseAccount, Bundler, SmartAccount};
use crate::config::{parse_config, Config};
use crate::erc4337::{Execution, PackedUserOperation};
use crate::execution::ExecutionHelper;
use alloy::network::EthereumWallet;
use alloy::primitives::{address, bytes, U256};
use alloy::signers::local::PrivateKeySigner;
use alloy::transports::http::reqwest::Url;
use clap::Parser;
use std::error::Error as StdError;
use std::path::PathBuf;
use std::sync::Arc;
use std::{fs, str::FromStr};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the JSON input file
    #[arg(short, long)]
    config: PathBuf,
    #[arg(short, long)]
    private_key: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let args = Args::parse();
    let config = parse_config(args.config).unwrap();

    run(config, args.private_key).await
}

async fn run(config: Config, priv_key: String) -> Result<(), Box<dyn StdError>> {
    let signer = PrivateKeySigner::from_str(&priv_key)?;
    let wallet = EthereumWallet::from(signer);
    let rpc_url = Url::parse("http://localhost:8545");

    println!("Hello LazyAccount");

    let account = SmartAccount::new().with_url(rpc_url?, &wallet);

    let account_address = address!("70997970C51812dc3A010C7d01b50e0d17dc79C8");
    let validator = address!("fB43116489394D843B2B29a7F6aa3eC0d590d795");

    let nonce = account.get_nonce(validator).await?;

    let execution = account.encode_execution(vec![Execution {
        target: validator,
        value: U256::ZERO,
        callData: bytes!("4141"),
    }]);

    let userop = PackedUserOperation::new()
        .with_sender(account_address)
        .with_nonce(nonce)
        .with_calldata(execution);
    account.send_user_op(userop).await?;

    Ok(())
}
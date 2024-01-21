//! Example on how to interact with a deployed `stylus-hello-world` program using defaults.
//! This example uses ethers-rs to instantiate the program using a Solidity ABI.
//! Then, it attempts to check the current counter value, increment it via a tx,
//! and check the value again. The deployed program is fully written in Rust and compiled to WASM
//! but with Stylus, it is accessible just as a normal Solidity smart contract is via an ABI.

use ethers::{
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::Address,
};
use eyre::eyre;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::Arc;

/// Your private key file path.
const ENV_PRIV_KEY_PATH: &str = "../wallet.txt";

/// Stylus RPC endpoint url.
const ENV_RPC_URL: &str = "https://stylus-testnet.arbitrum.io/rpc";

/// Deployed pragram address.
const ENV_PROGRAM_ADDRESS: &str = "0x05911DE732cb96CdA51C9a411072984D163EcAcf";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // let priv_key_path = std::env::var(ENV_PRIV_KEY_PATH)
    //     .map_err(|_| eyre!("No {} env var set", ENV_PRIV_KEY_PATH))?;
    // let rpc_url =
    //     std::env::var(ENV_RPC_URL).map_err(|_| eyre!("No {} env var set", ENV_RPC_URL))?;
    // let program_address = std::env::var(ENV_PROGRAM_ADDRESS)
    //     .map_err(|_| eyre!("No {} env var set", ENV_PROGRAM_ADDRESS))?;
    abigen!(
        Counter,
        r#"[
            function number() external view returns (uint256)
            function setNumber(uint256 number) external
            function increment() external
        ]"#
    );

    let provider = Provider::<Http>::try_from("https://stylus-testnet.arbitrum.io/rpc")?;
    let address: Address = "0x05911DE732cb96CdA51C9a411072984D163EcAcf".parse()?;

    let privkey = "e36d87e4ed845992fcdf3028d37f1cd062118cd8f1722f2855bf5d42cc50fa86";
    let wallet = LocalWallet::from_str(&privkey)?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.clone().with_chain_id(chain_id),
    ));

    let counter: Counter<SignerMiddleware<Provider<Http>, _>> = Counter::new(address, client);
    let num = counter.number().call().await;
    println!("Counter number value = {:?}", num);

    let _ = counter.increment().send().await?.await?;
    println!("Successfully incremented counter via a tx");

    let num = counter.number().call().await;
    println!("New counter number value = {:?}", num);
    Ok(())
}

fn read_secret_from_file(fpath: &str) -> eyre::Result<String> {
    let f = std::fs::File::open(fpath)?;
    let mut buf_reader = BufReader::new(f);
    let mut secret = String::new();
    buf_reader.read_line(&mut secret)?;
    Ok(secret.trim().to_string())
}

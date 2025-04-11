use clap::Parser;
use serde::Deserialize;
use starknet::{
    accounts::{Account, ExecutionEncoding, SingleOwnerAccount},
    core::utils::get_selector_from_name,
    providers::SequencerGatewayProvider,
    signers::{LocalWallet, SigningKey},
};

use starknet::core::types::Call as StarknetCall;

use starknet_types_core::felt::Felt;
use std::io::prelude::*;
use std::{fs::File, path::Path};
use url::Url;

#[derive(Deserialize, Debug)]
pub struct Call {
    pub to: String,
    pub entrypoint: String,
    pub calldata: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Fee {
    pub max_amount: String,
    pub max_price_per_unit: String,
}

#[derive(Deserialize, Debug)]
pub struct ResourceBounds {
    pub l2_gas: Fee,
    pub l1_gas: Fee,
    pub l1_data_gas: Fee
} 

#[derive(Deserialize, Debug)]
pub struct TxV3 {
    pub version: String,
    pub sender_address: String,
    pub tip: String,
    pub resource_bounds: ResourceBounds,
    pub paymaster_data: Vec<String>,
    pub chain_id: String,
    pub nonce: String,
    pub data_availability_mode: String,
    pub account_deployment_data: Vec<String>,
    pub calls: Vec<Call>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// JSON input file: Tx in JSON format
    #[arg(short, long)]
    json: String,
}

#[tokio::main]
async fn main() {
    let args: Args = Args::parse();

    // Read Tx from JSON file
    let path = Path::new(args.json.as_str());
    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let tx = serde_json::from_str::<TxV3>(&data).unwrap();

    // Create Sequencer Gateway Provider
    let provider = SequencerGatewayProvider::new(
        Url::parse("http://127.0.0.1:5050/gateway").unwrap(),
        Url::parse("http://127.0.0.1:5050/feeder_gateway").unwrap(),
        Felt::from_hex(&tx.chain_id).unwrap(),
    );

    // Create Signer
    let private_key =
        Felt::from_hex("0139fe4d6f02e666e86a6f58e65060f115cd3c185bd9e98bd829636931458f79").unwrap();
    let pkey = SigningKey::from_secret_scalar(private_key);
    let signer = LocalWallet::from_signing_key(pkey);

    // Create Account
    let account = SingleOwnerAccount::new(
        provider,
        signer,
        Felt::from_hex(&tx.sender_address).unwrap(),
        Felt::from_hex(&tx.chain_id).unwrap(),
        ExecutionEncoding::New,
    );

    let calls = tx
        .calls
        .iter()
        .map(|c| StarknetCall {
            to: Felt::from_hex(&c.to).unwrap(),
            selector: get_selector_from_name(&c.entrypoint).unwrap(),
            calldata: c
                .calldata
                .iter()
                .map(|d| Felt::from_hex(d).unwrap())
                .collect(),
        })
        .collect();

    let execution = account
        .execute_v3(calls)
        .l1_gas(u64::from_str_radix(&tx.resource_bounds.l1_gas.max_amount.trim_start_matches("0x"), 16).unwrap())
        .l1_gas_price(u128::from_str_radix(&tx.resource_bounds.l1_gas.max_price_per_unit.trim_start_matches("0x"), 16).unwrap())
        .l2_gas(u64::from_str_radix(&tx.resource_bounds.l2_gas.max_amount.trim_start_matches("0x"), 16).unwrap())
        .l2_gas_price(u128::from_str_radix(&tx.resource_bounds.l2_gas.max_price_per_unit.trim_start_matches("0x"), 16).unwrap())
        .l1_data_gas(u64::from_str_radix(&tx.resource_bounds.l1_data_gas.max_amount.trim_start_matches("0x"), 16).unwrap())
        .l1_data_gas_price(u128::from_str_radix(&tx.resource_bounds.l1_data_gas.max_price_per_unit.trim_start_matches("0x"), 16).unwrap())
        .nonce(Felt::from_hex_unchecked(&tx.nonce));
    

    let hash = execution.prepared().unwrap().transaction_hash(false);
    println!("Transaction hash: {}", hash.to_biguint());
    println!("Transaction hash: {}", hash.to_hex_string());
    println!("Transaction hash: {}", hash.to_fixed_hex_string());
}

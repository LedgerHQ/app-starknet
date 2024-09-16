use clap::Parser;
use serde::Deserialize;
use starknet::{
    accounts::{Account, ExecutionEncoding, SingleOwnerAccount},
    core::{chain_id, types::Call, utils::get_selector_from_name},
    providers::SequencerGatewayProvider,
    signers::{LocalWallet, SigningKey},
};
use starknet_types_core::felt::Felt;
use std::io::prelude::*;
use std::{fs::File, path::Path};
use url::Url;
//use ledger_lib::Transport;

#[derive(Deserialize, Debug)]
pub struct MyCall {
    pub to: String,
    pub entrypoint: String,
    pub calldata: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct TxV3 {
    pub url: String,
    pub version: u8,
    pub sender_address: String,
    pub tip: String,
    pub l1_gas_bounds: String,
    pub l2_gas_bounds: String,
    pub paymaster_data: Vec<String>,
    pub chain_id: String,
    pub nonce: String,
    pub data_availability_mode: String,
    pub account_deployment_data: Vec<String>,
    pub calls: Vec<MyCall>,
}

#[derive(Deserialize, Debug)]
pub struct TxV1 {
    pub url: String,
    pub version: u8,
    pub sender_address: String,
    pub max_fee: String,
    pub chain_id: String,
    pub nonce: String,
    pub calls: Vec<MyCall>,
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

    let path = Path::new(args.json.as_str());

    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let tx = serde_json::from_str::<TxV1>(&data).unwrap();

    // print chain id MAINET in hexadecimal
    //println!("Chain ID: {:x}", chain_id::MAINNET);

    let provider = SequencerGatewayProvider::new(
        Url::parse("http://127.0.0.1:5050/gateway").unwrap(),
        Url::parse("http://127.0.0.1:5050/feeder_gateway").unwrap(),
        chain_id::MAINNET,
    );

    let private_key =
        Felt::from_hex("0139fe4d6f02e666e86a6f58e65060f115cd3c185bd9e98bd829636931458f79").unwrap();
    let pkey = SigningKey::from_secret_scalar(private_key);
    let signer = LocalWallet::from_signing_key(pkey);

    let account = SingleOwnerAccount::new(
        provider,
        signer.clone(),
        Felt::from_hex(&tx.sender_address).unwrap(),
        Felt::from_hex(&tx.chain_id).unwrap(),
        ExecutionEncoding::New,
    );

    // let execution = account
    //     .execute_v3(vec![Call {
    //         to: to_address,
    //         selector: get_selector_from_name("transfer").unwrap(),
    //         calldata: vec![account.address(), Felt::from_dec_str("1000").unwrap()],
    //     }])
    //     .gas(0)
    //     .gas_price(0)
    //     .nonce(Felt::ONE);

    let calls = tx
        .calls
        .iter()
        .map(|c| Call {
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
        .execute_v1(calls)
        .max_fee(Felt::from_dec_str(&tx.max_fee).unwrap())
        .nonce(Felt::from_dec_str(&tx.nonce).unwrap());

    let hash = execution.prepared().unwrap().transaction_hash(false);
    println!("Transaction hash: {}", hash.to_biguint());
    println!("Transaction hash: {}", hash.to_hex_string());
    println!("Transaction hash: {}", hash.to_fixed_hex_string());
}

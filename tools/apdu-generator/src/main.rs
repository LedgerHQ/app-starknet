use clap::Parser;
use std::fs::File;
use std::io::prelude::*;

/// Utility to generate APDUs for Tx blur or clear signing with Starknet Nano application
/// (see https://docs.starknet.io/documentation/architecture_and_concepts/Blocks/transactions/#invoke_transaction_version_1)
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Tx in JSON format
    #[arg(short, long)]
    json: String,

    /// APDU CLA
    #[arg(short, long, default_value_t = 0x5A)]
    cla: u8,

    /// APDU INS
    #[arg(short, long, default_value_t = 0x03)]
    ins: u8,
}

use apdu_generator::{apdu::Apdu, builder, types::Tx};

// Derivation path
const PATH: &str = "m/2645'/1195502025'/1148870696'/0'/0'/0";
// Hash
// const HASH: &str = "0x55b8f28706a5008d3103bcb2bfa6356e56b95c34fed265c955846670a6bb4ef";

fn main() {
    let args: Args = Args::parse();

    let mut file = File::open(args.json).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let mut tx: Tx = serde_json::from_str(&data).unwrap();
    tx.calls.reverse();

    let mut apdus: Vec<Apdu> = Vec::new();

    let dpath_apdu = builder::derivation_path(PATH, args.cla, args.ins.into(), 0);
    apdus.push(dpath_apdu.clone());

    let tx_data_apdu = builder::tx_data(&tx, args.cla, args.ins.into(), 1);
    apdus.push(tx_data_apdu.clone());

    let tx_data_apdu = builder::paymaster_data(&tx.paymaster_data, args.cla, args.ins.into(), 2);
    apdus.push(tx_data_apdu.clone());

    let tx_data_apdu =
        builder::accound_deployment_data(&tx.account_deployment_data, args.cla, args.ins.into(), 3);
    apdus.push(tx_data_apdu.clone());

    let tx_data_apdu = builder::calls_nb(&tx.calls, args.cla, args.ins.into(), 4);
    apdus.push(tx_data_apdu.clone());

    while tx.calls.len() > 0 {
        let call = tx.calls.pop().unwrap();
        let mut call_apdu = builder::call(&call, args.cla, args.ins.into(), 5);
        apdus.append(&mut call_apdu);
    }

    let mut json_out = File::create("apdu.json").unwrap();
    let mut raw_out = File::create("apdu.dat").unwrap();
    for a in apdus.iter() {
        println!("=> {}", a);
        writeln!(raw_out, "=> {}", a).unwrap();
    }
    writeln!(
        json_out,
        "{}",
        serde_json::to_string_pretty(&apdus).unwrap()
    )
    .unwrap();
}

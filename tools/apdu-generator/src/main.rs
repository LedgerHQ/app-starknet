use clap::Parser;
use std::io::prelude::*;
use std::{fs::File, path::Path};

/// Utility to generate APDUs for Tx blur or clear signing with Starknet Nano application
/// (see https://docs.starknet.io/documentation/architecture_and_concepts/Blocks/transactions/#invoke_transaction_version_1)
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// JSON input file: Hash or Tx in JSON format
    #[arg(short, long)]
    json: String,

    /// APDU CLA
    #[arg(short, long, default_value_t = 0x5A)]
    cla: u8,

    /// APDU INS (2 for signHash(), 3 for signTx(), 4 for signTxV1())
    #[arg(short, long)]
    ins: u8,
}

use apdu_generator::{
    apdu::Apdu,
    builder,
    types::{Hash, Tx, TxV1, TxV3},
};

// Derivation path
const PATH: &str = "m/2645'/1195502025'/1148870696'/0'/0'/0";

fn main() {
    let args: Args = Args::parse();

    let path = Path::new(args.json.as_str());

    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let mut apdus: Vec<Apdu> = Vec::new();

    match args.ins {
        2 => {
            let hash = serde_json::from_str::<Hash>(&data).unwrap();

            let dpath_apdu = builder::derivation_path(PATH, args.cla, args.ins.into(), 0);
            apdus.push(dpath_apdu.clone());

            let apdu = builder::hash_to_apdu(&hash.hash, args.cla, args.ins.into(), 1, true);
            apdus.push(apdu.clone());
        }
        3 | 4 => {
            let tx = match args.ins {
                3 => {
                    let t = serde_json::from_str::<TxV3>(&data).unwrap();
                    Tx::V3(t)
                }
                4 => {
                    let t = serde_json::from_str::<TxV1>(&data).unwrap();
                    Tx::V1(t)
                }
                _ => panic!("Invalid INS"),
            };

            let dpath_apdu = builder::derivation_path(PATH, args.cla, args.ins.into(), 0);
            apdus.push(dpath_apdu.clone());

            let tx_data_apdu = builder::tx_data(&tx, args.cla, args.ins.into(), 1);
            apdus.push(tx_data_apdu.clone());

            match tx {
                Tx::V1(mut tx) => {
                    let tx_data_apdu = builder::calls_nb(&tx.calls, args.cla, args.ins.into(), 2);
                    apdus.push(tx_data_apdu.clone());
                    tx.calls.reverse();
                    while tx.calls.len() > 0 {
                        let call = tx.calls.pop().unwrap();
                        let mut call_apdu = builder::call(&call, args.cla, args.ins.into(), 3);
                        apdus.append(&mut call_apdu);
                    }
                }
                Tx::V3(mut tx) => {
                    let tx_data_apdu =
                        builder::paymaster_data(&tx.paymaster_data, args.cla, args.ins.into(), 2);
                    apdus.push(tx_data_apdu.clone());

                    tx.calls.reverse();
                    let tx_data_apdu = builder::accound_deployment_data(
                        &tx.account_deployment_data,
                        args.cla,
                        args.ins.into(),
                        3,
                    );
                    apdus.push(tx_data_apdu.clone());

                    let tx_data_apdu = builder::calls_nb(&tx.calls, args.cla, args.ins.into(), 4);
                    apdus.push(tx_data_apdu.clone());

                    while tx.calls.len() > 0 {
                        let call = tx.calls.pop().unwrap();
                        let mut call_apdu = builder::call(&call, args.cla, args.ins.into(), 5);
                        apdus.append(&mut call_apdu);
                    }
                }
            }
        }
        _ => panic!("Invalid INS"),
    }

    let out_name = path.file_name().unwrap().to_str().unwrap();
    let out_name_with_ext_apdu = format!("{}.dat", out_name[0..out_name.len() - 5].to_string());
    let out_name_with_ext_json = format!("{}.json", out_name[0..out_name.len() - 5].to_string());

    let json_out_name = path
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("apdu_samples")
        .join(out_name_with_ext_json.clone());

    let raw_out_name = path
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("apdu_samples")
        .join(out_name_with_ext_apdu.clone());

    println!(
        "Writing APDUs to {:?} and {:?}",
        json_out_name, raw_out_name
    );

    let mut json_out = File::create(json_out_name).unwrap();
    let mut raw_out = File::create(raw_out_name).unwrap();
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

use apdu_generator::types::FieldElement;
use clap::Parser;
use std::io::prelude::*;
use std::{fs::File, path::Path};

/// Utility to generate APDUs for Tx blur or clear signing with Starknet Nano application
/// (see https://docs.starknet.io/documentation/architecture_and_concepts/Blocks/transactions/#invoke_transaction_version_1)
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// JSON input file: Derivation path or Hash or Tx in JSON format
    #[arg(short, long)]
    json: String,

    /// APDU CLA
    #[arg(short, long, default_value_t = 0x5A)]
    cla: u8,

    /// APDU INS (1 for getPubKey(), 2 for signHash(), 3 for signTx(), 4 for signTxV1())
    #[arg(short, long)]
    ins: u8,
}

use apdu_generator::{
    apdu::Apdu,
    builder,
    types::{DeployAccountV1, DeployAccountV3, Dpath, Hash, InvokeV1, InvokeV3, Tx},
};

fn main() {
    let args: Args = Args::parse();

    let path = Path::new(args.json.as_str());

    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let mut apdus: Vec<Apdu> = Vec::new();

    match args.ins {
        1 => {
            let path = serde_json::from_str::<Dpath>(&data).unwrap();
            println!("Derivation path: {:?}", path);
            let dpath_apdu = builder::derivation_path(&path.dpath, args.cla, args.ins.into(), 0);
            apdus.push(dpath_apdu.clone());
        }
        2 => {
            let hash = serde_json::from_str::<Hash>(&data).unwrap();

            let dpath_apdu = builder::derivation_path(&hash.dpath, args.cla, args.ins.into(), 0);
            apdus.push(dpath_apdu.clone());

            let apdu = builder::hash_to_apdu(&hash.hash, args.cla, args.ins.into(), 1, true);
            apdus.push(apdu.clone());
        }
        3 | 4 | 5 | 6 => {
            let tx = match args.ins {
                3 => {
                    let t = match serde_json::from_str::<InvokeV3>(&data) {
                        Ok(t) => t,
                        Err(_) => {
                            panic!("Invalid TxV3 format");
                        }
                    };
                    Tx::V3(t)
                }
                4 => {
                    let t = match serde_json::from_str::<InvokeV1>(&data) {
                        Ok(t) => t,
                        Err(_) => {
                            panic!("Invalid TxV1 format");
                        }
                    };
                    Tx::V1(t)
                }
                5 => {
                    let t = match serde_json::from_str::<DeployAccountV3>(&data) {
                        Ok(t) => t,
                        Err(_) => {
                            panic!("Invalid DeployAccountV3 format");
                        }
                    };
                    Tx::DeployV3(t)
                }
                6 => {
                    let t = match serde_json::from_str::<DeployAccountV1>(&data) {
                        Ok(t) => t,
                        Err(_) => {
                            panic!("Invalid TxV1 format");
                        }
                    };
                    Tx::DeployV1(t)
                }
                _ => panic!("Invalid INS"),
            };

            match tx {
                Tx::V1(mut tx) => {
                    let dpath_apdu =
                        builder::derivation_path(&tx.dpath, args.cla, args.ins.into(), 0);
                    apdus.push(dpath_apdu.clone());

                    let tx_data_apdu =
                        builder::tx_fields_invoke_v1(&tx, args.cla, args.ins.into(), 1);
                    apdus.push(tx_data_apdu.clone());

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
                    let dpath_apdu =
                        builder::derivation_path(&tx.dpath, args.cla, args.ins.into(), 0);
                    apdus.push(dpath_apdu.clone());

                    let tx_data_apdu =
                        builder::tx_fields_invoke_v3(&tx, args.cla, args.ins.into(), 1);
                    apdus.push(tx_data_apdu.clone());

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
                Tx::DeployV3(tx) => {
                    let dpath_apdu =
                        builder::derivation_path(&tx.dpath, args.cla, args.ins.into(), 0);
                    apdus.push(dpath_apdu.clone());

                    let tx_data_apdu =
                        builder::tx_fields_deploy_v3(&tx, args.cla, args.ins.into(), 1);
                    apdus.push(tx_data_apdu.clone());

                    let tip: FieldElement = FieldElement::try_from(tx.tip.as_str()).unwrap();
                    let l1_gas_bounds: FieldElement =
                        FieldElement::try_from(tx.l1_gas_bounds.as_str()).unwrap();
                    let l2_gas_bounds: FieldElement =
                        FieldElement::try_from(tx.l2_gas_bounds.as_str()).unwrap();

                    let fees: Vec<FieldElement> = vec![tip, l1_gas_bounds, l2_gas_bounds];

                    let fees_apdu = builder::tx_fees(&fees, args.cla, args.ins.into(), 2);
                    apdus.push(fees_apdu.clone());

                    let paymaster_apdu =
                        builder::paymaster_data(&tx.paymaster_data, args.cla, args.ins.into(), 3);
                    apdus.push(paymaster_apdu.clone());

                    let mut constructor_calldata: Vec<FieldElement> = Default::default();
                    for c in tx.constructor_calldata.iter() {
                        let fe: FieldElement = FieldElement::try_from(c.as_str()).unwrap();
                        constructor_calldata.push(fe);
                    }

                    let mut constructor_calldata_apdus = builder::constructor_calldata(
                        &constructor_calldata,
                        args.cla,
                        args.ins.into(),
                        4,
                    );
                    apdus.append(&mut constructor_calldata_apdus);
                }
                Tx::DeployV1(tx) => {
                    let dpath_apdu =
                        builder::derivation_path(&tx.dpath, args.cla, args.ins.into(), 0);
                    apdus.push(dpath_apdu.clone());

                    let tx_data_apdu =
                        builder::tx_fields_deploy_v1(&tx, args.cla, args.ins.into(), 1);
                    apdus.push(tx_data_apdu.clone());

                    let max_fee: FieldElement =
                        FieldElement::try_from(tx.max_fee.as_str()).unwrap();
                    let fees: Vec<FieldElement> = vec![max_fee];
                    let fees_apdu = builder::tx_fees(&fees, args.cla, args.ins.into(), 2);
                    apdus.push(fees_apdu.clone());

                    let mut constructor_calldata: Vec<FieldElement> = Default::default();
                    for c in tx.constructor_calldata.iter() {
                        let fe: FieldElement = FieldElement::try_from(c.as_str()).unwrap();
                        constructor_calldata.push(fe);
                    }

                    let mut constructor_calldata_apdus = builder::constructor_calldata(
                        &constructor_calldata,
                        args.cla,
                        args.ins.into(),
                        3,
                    );
                    apdus.append(&mut constructor_calldata_apdus);
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
        .join("apdu")
        .join(out_name_with_ext_json.clone());

    let raw_out_name = path
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("apdu")
        .join(out_name_with_ext_apdu.clone());

    println!(
        "Writing APDUs to {:?} and {:?}",
        json_out_name, raw_out_name
    );

    if let Some(parent) = json_out_name.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }

    let mut json_out = File::create_new(json_out_name).unwrap();
    let mut raw_out = File::create_new(raw_out_name).unwrap();
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

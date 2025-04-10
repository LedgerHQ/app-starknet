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
}

use apdu_generator::{
    apdu::Apdu,
    builder,
    types::{DeployAccountV1, DeployAccountV3, Dpath, Hash, InvokeV1, InvokeV3, Tx, Ins},
};

fn main() {
    let args: Args = Args::parse();

    let path = Path::new(args.json.as_str());

    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let mut apdus: Vec<Apdu> = Vec::new();

    if let Ok(path) = serde_json::from_str::<Dpath>(&data) {
        println!("Derivation path: {:?}", path);
        let dpath_apdu = builder::derivation_path(&path.dpath_getpubkey, args.cla, Ins::GetPubkey, 0);
        apdus.push(dpath_apdu.clone());
    } else if let Ok(hash) = serde_json::from_str::<Hash>(&data) {
        let dpath_apdu = builder::derivation_path(&hash.dpath_signhash, args.cla, Ins::SignHash, 0);
        apdus.push(dpath_apdu.clone());

        let apdu = builder::hash_to_apdu(&hash.hash, args.cla, Ins::SignHash, 1, true);
        apdus.push(apdu.clone());
    } else if let Ok(tx) = serde_json::from_str::<InvokeV3>(&data)
        .map(Tx::V3)
        .or_else(|_| serde_json::from_str::<InvokeV1>(&data).map(Tx::V1))
        .or_else(|_| serde_json::from_str::<DeployAccountV3>(&data).map(Tx::DeployV3))
        .or_else(|_| serde_json::from_str::<DeployAccountV1>(&data).map(Tx::DeployV1))
    {
        match tx {
            Tx::V1(mut tx) => {
                let dpath_apdu =
                    builder::derivation_path(&tx.dpath, args.cla, Ins::SignTxV1, 0);
                apdus.push(dpath_apdu.clone());

                let tx_data_apdu =
                    builder::tx_fields_invoke_v1(&tx, args.cla, Ins::SignTxV1, 1);
                apdus.push(tx_data_apdu.clone());

                let tx_data_apdu = builder::calls_nb(&tx.calls, args.cla, Ins::SignTxV1, 2);
                apdus.push(tx_data_apdu.clone());
                tx.calls.reverse();
                while tx.calls.len() > 0 {
                    let call = tx.calls.pop().unwrap();
                    let mut call_apdu = builder::call(&call, args.cla, Ins::SignTxV1, 3);
                    apdus.append(&mut call_apdu);
                }
            }
            Tx::V3(mut tx) => {
                let dpath_apdu =
                    builder::derivation_path(&tx.dpath, args.cla, Ins::SignTx, 0);
                apdus.push(dpath_apdu.clone());

                let tx_data_apdu =
                    builder::tx_fields_invoke_v3(&tx, args.cla, Ins::SignTx, 1);
                apdus.push(tx_data_apdu.clone());

                let tip: FieldElement = FieldElement::try_from(tx.tip.as_str()).unwrap();
                let l1_gas_bounds: FieldElement =
                    FieldElement::try_from(tx.l1_gas_bounds.as_str()).unwrap();
                let l2_gas_bounds: FieldElement =
                    FieldElement::try_from(tx.l2_gas_bounds.as_str()).unwrap();
                let mut fees: Vec<FieldElement> = vec![tip, l1_gas_bounds, l2_gas_bounds];
                if let Some(l1_data_gas) = tx.l1_data_gas {
                    let l1_data_gas: FieldElement =
                        FieldElement::try_from(l1_data_gas.as_str()).unwrap();
                    fees.push(l1_data_gas);
                }
                let fees_apdu = builder::tx_fees(&fees, args.cla, Ins::SignTx, 2);
                apdus.push(fees_apdu.clone());

                let tx_data_apdu =
                    builder::paymaster_data(&tx.paymaster_data, args.cla, Ins::SignTx, 3);
                apdus.push(tx_data_apdu.clone());

                tx.calls.reverse();
                let tx_data_apdu = builder::accound_deployment_data(
                    &tx.account_deployment_data,
                    args.cla,
                    Ins::SignTx,
                    4,
                );
                apdus.push(tx_data_apdu.clone());

                let tx_data_apdu = builder::calls_nb(&tx.calls, args.cla, Ins::SignTx, 5);
                apdus.push(tx_data_apdu.clone());

                while tx.calls.len() > 0 {
                    let call = tx.calls.pop().unwrap();
                    let mut call_apdu = builder::call(&call, args.cla, Ins::SignTx, 6);
                    apdus.append(&mut call_apdu);
                }
            }
            Tx::DeployV3(tx) => {
                let dpath_apdu =
                    builder::derivation_path(&tx.dpath, args.cla, Ins::SignDeployAccount, 0);
                apdus.push(dpath_apdu.clone());

                let tx_data_apdu =
                    builder::tx_fields_deploy_v3(&tx, args.cla, Ins::SignDeployAccount, 1);
                apdus.push(tx_data_apdu.clone());

                let tip: FieldElement = FieldElement::try_from(tx.tip.as_str()).unwrap();
                let l1_gas_bounds: FieldElement =
                    FieldElement::try_from(tx.l1_gas_bounds.as_str()).unwrap();
                let l2_gas_bounds: FieldElement =
                    FieldElement::try_from(tx.l2_gas_bounds.as_str()).unwrap();
                let mut fees: Vec<FieldElement> = vec![tip, l1_gas_bounds, l2_gas_bounds];
                if let Some(l1_data_gas) = tx.l1_data_gas {
                    let l1_data_gas: FieldElement =
                        FieldElement::try_from(l1_data_gas.as_str()).unwrap();
                    fees.push(l1_data_gas);
                }
                let fees_apdu = builder::tx_fees(&fees, args.cla, Ins::SignDeployAccount, 2);
                apdus.push(fees_apdu.clone());

                let paymaster_apdu =
                    builder::paymaster_data(&tx.paymaster_data, args.cla, Ins::SignDeployAccount, 3);
                apdus.push(paymaster_apdu.clone());

                let mut constructor_calldata: Vec<FieldElement> = Default::default();
                for c in tx.constructor_calldata.iter() {
                    let fe: FieldElement = FieldElement::try_from(c.as_str()).unwrap();
                    constructor_calldata.push(fe);
                }

                let mut constructor_calldata_apdus = builder::constructor_calldata(
                    &constructor_calldata,
                    args.cla,
                    Ins::SignDeployAccount,
                    4,
                );
                apdus.append(&mut constructor_calldata_apdus);
            }
            Tx::DeployV1(tx) => {
                let dpath_apdu =
                    builder::derivation_path(&tx.dpath, args.cla, Ins::SignDeployAccountV1, 0);
                apdus.push(dpath_apdu.clone());

                let tx_data_apdu =
                    builder::tx_fields_deploy_v1(&tx, args.cla, Ins::SignDeployAccountV1, 1);
                apdus.push(tx_data_apdu.clone());

                let max_fee: FieldElement =
                    FieldElement::try_from(tx.max_fee.as_str()).unwrap();
                let fees: Vec<FieldElement> = vec![max_fee];
                let fees_apdu = builder::tx_fees(&fees, args.cla, Ins::SignDeployAccountV1, 2);
                apdus.push(fees_apdu.clone());

                let mut constructor_calldata: Vec<FieldElement> = Default::default();
                for c in tx.constructor_calldata.iter() {
                    let fe: FieldElement = FieldElement::try_from(c.as_str()).unwrap();
                    constructor_calldata.push(fe);
                }

                let mut constructor_calldata_apdus = builder::constructor_calldata(
                    &constructor_calldata,
                    args.cla,
                    Ins::SignDeployAccountV1,
                    3,
                );
                apdus.append(&mut constructor_calldata_apdus);
            }
        }
    } else {
        panic!("Invalid input format");
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

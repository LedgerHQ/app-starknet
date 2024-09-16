use clap::Parser;
use starknet::{core::crypto::Signature, signers::VerifyingKey};
use starknet_types_core::felt::Felt;
//use ledger_lib::Transport;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Hash of the transaction
    #[arg(short, long)]
    txhash: String,

    /// Public key of the device
    #[arg(short, long)]
    pkey: String,

    /// R value of the signature
    #[arg(short, long)]
    r: String,

    /// S value of the signature
    #[arg(short, long)]
    s: String,
}

#[tokio::main]
async fn main() {
    let args: Args = Args::parse();

    /* Check signature (device) */

    // Initialise provider
    //let mut p = ledger_lib::LedgerProvider::init().await;

    // Fetch list of available devices
    //let devices = p.list(ledger_lib::Filters::Hid).await.unwrap();

    // Connect to device
    //let d = &devices[0];

    // Connect to the device using the index offset
    //let device_handle = match p.connect(d.clone()).await {
    //    Ok(v) => v,
    //    Err(e) => {
    //        println!("Failed to connect to device {:?}: {:?}", d, e);
    //        return;
    //    }
    //};
    //let mut buff = [0u8; 256];

    let device_public_key = VerifyingKey::from_scalar(Felt::from_hex_unchecked(args.pkey.as_str()));

    let tx_hash = Felt::from_hex_unchecked(args.txhash.as_str());

    let device_signature = Signature {
        r: Felt::from_hex_unchecked(args.r.as_str()),
        s: Felt::from_hex_unchecked(args.s.as_str()),
    };

    let device_verify = device_public_key
        .verify(&tx_hash, &device_signature)
        .unwrap();
    println!("{}", device_verify);
}

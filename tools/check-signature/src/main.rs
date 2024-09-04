use clap::Parser;
use starknet::{
    accounts::{Account, Call, ExecutionEncoding, SingleOwnerAccount},
    core::{chain_id, crypto::Signature, utils::get_selector_from_name},
    providers::SequencerGatewayProvider,
    signers::{LocalWallet, Signer, SigningKey, VerifyingKey},
};
use starknet_types_core::felt::Felt;
use url::Url;

use ledger_lib::Transport;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// public key in hexadecimal format (32 bytes)
    #[arg(short, long)]
    pkey: String,

    /// Tx hash in hexadecimal format (32 bytes)
    #[arg(short, long)]
    txhash: String,

    /// Signature r value in hexadecimal format (32 bytes)
    #[arg(short, long)]
    r: String,

    /// Signature s value in hexadecimal format (32 bytes)
    #[arg(short, long)]
    s: String,
}

#[tokio::main]
async fn main() {
    let args: Args = Args::parse();

    /*
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
        Felt::from_hex("07e00d496e324876bbc8531f2d9a82bf154d1a04a50218ee74cdd372f75a551a").unwrap(),
        chain_id::MAINNET,
        ExecutionEncoding::New,
    );

    let tst_token_address =
        Felt::from_hex("049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7").unwrap();

    let execution = account
        .execute_v3(vec![Call {
            to: tst_token_address,
            selector: get_selector_from_name("transfer").unwrap(),
            calldata: vec![account.address(), Felt::from_dec_str("1000").unwrap()],
        }])
        .gas(0)
        .gas_price(0)
        .nonce(Felt::ONE);

    let prepared = execution.prepared().unwrap();

    let hash = prepared.transaction_hash(false);

    println!("Transaction hash: {}", hash.to_biguint());
    println!("Transaction hash: {}", hash.to_hex_string());
    println!("Transaction hash: {}", hash.to_fixed_hex_string());
    */

    /* Check signature (ref) */
    //let signature = signer.sign_hash(&hash).await.unwrap();

    //let public_key = signer.get_public_key().await.unwrap();

    //let verify = public_key.verify(&hash, &signature).unwrap();

    //println!("Verify: {}", verify);

    /* Check signature (device) */

    // Initialise provider
    let mut p = ledger_lib::LedgerProvider::init().await;

    // Fetch list of available devices
    let devices = p.list(ledger_lib::Filters::Hid).await.unwrap();

    // Connect to device
    let d = &devices[0];

    // Connect to the device using the index offset
    let device_handle = match p.connect(d.clone()).await {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to connect to device {:?}: {:?}", d, e);
            return;
        }
    };
    let mut buff = [0u8; 256];

    let device_public_key = VerifyingKey::from_scalar(Felt::from_hex_unchecked(args.pkey.as_str()));

    let tx_hash = Felt::from_hex_unchecked(args.txhash.as_str());

    let device_signature = Signature {
        r: Felt::from_hex_unchecked(args.r.as_str()),
        s: Felt::from_hex_unchecked(args.s.as_str()),
    };

    let device_verify = device_public_key
        .verify(&tx_hash, &device_signature)
        .unwrap();
    println!("Verify: {}", device_verify);
}

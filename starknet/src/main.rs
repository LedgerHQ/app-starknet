#![no_std]
#![no_main]

mod context;
mod crypto;
mod display;
mod erc20;
mod settings;
mod transaction;
mod types;

extern crate alloc;
use alloc::{format, vec::Vec};

use context::{
    Ctx, DeployAccountTransactionV1, DeployAccountTransactionV3, InvokeTransactionV1,
    InvokeTransactionV3, RequestType, Transaction,
};
use ledger_device_sdk::io;
use types::FieldElement;

use settings::Settings;
use transaction::SetCallStep;

ledger_device_sdk::set_panic!(ledger_device_sdk::exiting_panic);

#[cfg(any(target_os = "stax", target_os = "flex"))]
use ledger_device_sdk::nbgl::init_comm;

#[cfg(any(target_os = "nanosplus", target_os = "nanox"))]
const PARSING_STEP_TX_WORDING: &str = "Parsing transaction";

#[cfg(any(target_os = "stax", target_os = "flex"))]
const PARSING_STEP_TX_WORDING: &str = "Start parsing the transaction";

const PARSING_STEP_CALL_WORDING: &str = "Parsing call ";

#[no_mangle]
extern "C" fn sample_main() {
    // Init comm and set the expected CLA byte for the application
    let mut comm = io::Comm::new().set_expected_cla(0x5A);

    let mut ctx = Ctx::new();

    #[cfg(not(any(target_os = "stax", target_os = "flex")))]
    {
        loop {
            // Wait for either a specific button push to exit the app
            // or an APDU command
            if let io::Event::Command(ins) = display::main_ui(&mut comm) {
                handle_apdu(&mut comm, &ins, &mut ctx);
            }
        }
    }

    #[cfg(any(target_os = "stax", target_os = "flex"))]
    {
        // Initialize reference to Comm instance for NBGL
        // API calls.
        init_comm(&mut comm);

        ctx.home = display::main_ui_nbgl(&mut comm);

        ctx.home.show_and_return();
        loop {
            // Wait for an APDU command
            let ins: Ins = comm.next_command();
            handle_apdu(&mut comm, &ins, &mut ctx);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
enum Ins {
    GetVersion,
    GetPubkey {
        display: bool,
    },
    #[cfg(feature = "signhash")]
    SignHash,
    SignTx,
    SignTxV1,
    SignDeployAccount,
    SignDeployAccountV1,
    #[cfg(feature = "poseidon")]
    Poseidon,
}

impl TryFrom<io::ApduHeader> for Ins {
    type Error = io::StatusWords;
    fn try_from(header: io::ApduHeader) -> Result<Self, Self::Error> {
        match (header.ins, header.p1, header.p2) {
            (0, 0, 0) => Ok(Ins::GetVersion),
            (0, _, _) => Err(io::StatusWords::BadP1P2),
            (1, 0 | 1, 0) => Ok(Ins::GetPubkey {
                display: header.p1 != 0,
            }),
            (1, _, _) => Err(io::StatusWords::BadP1P2),
            #[cfg(feature = "signhash")]
            (2, _, _) => Ok(Ins::SignHash),
            (3, _, _) => Ok(Ins::SignTx),
            (4, _, _) => Ok(Ins::SignTxV1),
            (5, _, _) => Ok(Ins::SignDeployAccount),
            (6, _, _) => Ok(Ins::SignDeployAccountV1),
            #[cfg(feature = "poseidon")]
            (7, _, _) => Ok(Ins::Poseidon),
            (_, _, _) => Err(io::StatusWords::BadIns),
        }
    }
}

use ledger_device_sdk::io::Reply;

const SIG_LENGTH: u8 = 0x41;

fn send_data(comm: &mut io::Comm, data: Result<Option<Vec<u8>>, Reply>) {
    match data {
        Ok(data) => {
            if let Some(data) = data {
                comm.append(data.as_slice())
            }
            comm.reply_ok();
        }
        Err(sw) => comm.reply(sw),
    }
}

fn handle_apdu(comm: &mut io::Comm, ins: &Ins, ctx: &mut Ctx) {
    if comm.rx == 0 {
        send_data(comm, Err(io::StatusWords::NothingReceived.into()));
    }

    let apdu_header = comm.get_apdu_metadata();
    let mut data = match comm.get_data() {
        Ok(data) => data,
        Err(e) => {
            send_data(comm, Err(e.into()));
            return;
        }
    };
    let p1 = apdu_header.p1;
    let p2 = apdu_header.p2;

    let mut rdata: Vec<u8> = Vec::new();

    match ins {
        Ins::GetVersion => {
            let version_major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u8>().unwrap();
            let version_minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u8>().unwrap();
            let version_patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u8>().unwrap();

            rdata.extend_from_slice([version_major, version_minor, version_patch].as_slice());

            send_data(comm, Ok(Some(rdata)));
        }
        Ins::GetPubkey { display } => {
            ctx.reset();
            ctx.req_type = RequestType::GetPubkey;

            let res = crypto::set_derivation_path(&mut data, ctx);
            match res {
                Err(e) => {
                    send_data(comm, Err(e.into()));
                }
                Ok(()) => {
                    let pub_key = crypto::get_pubkey(ctx);
                    match pub_key {
                        Err(e) => {
                            send_data(comm, Err(Reply::from(e)));
                        }
                        Ok(key) => {
                            let ret = match display {
                                false => true,
                                true => display::pkey_ui(key.as_ref(), ctx),
                            };
                            if ret {
                                rdata.extend_from_slice(key.as_ref());
                                send_data(comm, Ok(Some(rdata)));
                            } else {
                                send_data(comm, Err(io::StatusWords::UserCancelled.into()));
                            }
                        }
                    }
                }
            }
        }
        #[cfg(feature = "signhash")]
        Ins::SignHash => match p1 {
            0 => {
                ctx.reset();
                ctx.req_type = RequestType::SignHash;

                match crypto::set_derivation_path(&mut data, ctx) {
                    Ok(()) => {
                        send_data(comm, Ok(None));
                    }
                    Err(e) => {
                        send_data(comm, Err(e.into()));
                    }
                }
            }
            _ => {
                let settings: Settings = Default::default();
                if settings.get_element(0) == 0 {
                    display::blind_signing_enable_ui(ctx);
                    send_data(comm, Err(io::StatusWords::UserCancelled.into()));
                } else {
                    ctx.hash = data.into();
                    match display::show_hash(ctx, false) {
                        true => {
                            crypto::sign_hash(ctx).unwrap();
                            rdata.extend_from_slice([0x41].as_slice());
                            rdata.extend_from_slice(ctx.signature.r.as_ref());
                            rdata.extend_from_slice(ctx.signature.s.as_ref());
                            rdata.extend_from_slice([ctx.signature.v].as_slice());
                            display::show_status(true, false, ctx);
                            send_data(comm, Ok(Some(rdata)));
                        }
                        false => {
                            display::show_status(false, false, ctx);
                            send_data(comm, Err(io::StatusWords::UserCancelled.into()));
                        }
                    }
                }
            }
        },
        Ins::SignTx => match p1 {
            0 => {
                ctx.reset();
                ctx.req_type = RequestType::SignTx;
                ctx.tx = Transaction::InvokeV3(InvokeTransactionV3::default());
                match crypto::set_derivation_path(&mut data, ctx) {
                    Ok(()) => {
                        send_data(comm, Ok(None));
                    }
                    Err(e) => {
                        send_data(comm, Err(e.into()));
                    }
                }
            }
            1 => {
                display::show_step(PARSING_STEP_TX_WORDING, ctx);
                transaction::set_tx_fields(data, &mut ctx.tx);
                send_data(comm, Ok(None));
            }
            2 => {
                #[cfg(any(target_os = "nanosplus", target_os = "nanox"))]
                display::show_step(PARSING_STEP_TX_WORDING, ctx);
                transaction::set_paymaster_data(data, p2, &mut ctx.tx);
                send_data(comm, Ok(None));
            }
            3 => {
                #[cfg(any(target_os = "nanosplus", target_os = "nanox"))]
                display::show_step(PARSING_STEP_TX_WORDING, ctx);
                transaction::set_account_deployment_data(data, p2, &mut ctx.tx);
                send_data(comm, Ok(None));
            }
            4 => {
                #[cfg(any(target_os = "nanosplus", target_os = "nanox"))]
                display::show_step(PARSING_STEP_TX_WORDING, ctx);
                let nb_calls: u8 = FieldElement::from(data).into();
                transaction::set_calldata_nb(&mut ctx.tx, nb_calls);
                send_data(comm, Ok(None));
            }
            5 => {
                #[cfg(any(target_os = "nanosplus", target_os = "nanox"))]
                {
                    if p2 == SetCallStep::End.into() {
                        display::show_step(
                            format!(
                                "{}{}/{}",
                                PARSING_STEP_CALL_WORDING,
                                ctx.tx.get_nb_received_calls(),
                                ctx.tx.get_nb_calls()
                            )
                            .as_str(),
                            ctx,
                        );
                    }
                }
                #[cfg(any(target_os = "stax", target_os = "flex"))]
                {
                    if p2 == SetCallStep::New.into() {
                        display::show_step(
                            format!(
                                "{}{}/{}",
                                PARSING_STEP_CALL_WORDING,
                                ctx.tx.get_nb_received_calls() + 1,
                                ctx.tx.get_nb_calls()
                            )
                            .as_str(),
                            ctx,
                        );
                    }
                }
                if let Some(err) = transaction::set_calldata(data, p2.into(), &mut ctx.tx).err() {
                    send_data(comm, Err(Reply(err as u16)));
                }
                if p2 == transaction::SetCallStep::End.into() {
                    match transaction::tx_complete(&mut ctx.tx) {
                        None => {
                            send_data(comm, Ok(None));
                        }
                        Some(hash) => {
                            ctx.hash = hash;
                            match display::show_tx(ctx) {
                                Some(approved) => match approved {
                                    true => {
                                        rdata.extend_from_slice(ctx.hash.value.as_ref());
                                        crypto::sign_hash(ctx).unwrap();
                                        rdata.extend_from_slice([SIG_LENGTH].as_slice());
                                        rdata.extend_from_slice(ctx.signature.r.as_ref());
                                        rdata.extend_from_slice(ctx.signature.s.as_ref());
                                        rdata.extend_from_slice([ctx.signature.v].as_slice());
                                        display::show_status(true, true, ctx);
                                        send_data(comm, Ok(Some(rdata)));
                                    }
                                    false => {
                                        display::show_status(false, true, ctx);
                                        send_data(comm, Err(io::StatusWords::UserCancelled.into()));
                                    }
                                },
                                None => {
                                    let settings: Settings = Default::default();
                                    if settings.get_element(0) == 0 {
                                        display::blind_signing_enable_ui(ctx);
                                        send_data(comm, Err(io::StatusWords::UserCancelled.into()));
                                    } else {
                                        match display::show_hash(ctx, true) {
                                            true => {
                                                rdata.extend_from_slice(ctx.hash.value.as_ref());
                                                crypto::sign_hash(ctx).unwrap();
                                                rdata.extend_from_slice([SIG_LENGTH].as_slice());
                                                rdata.extend_from_slice(ctx.signature.r.as_ref());
                                                rdata.extend_from_slice(ctx.signature.s.as_ref());
                                                rdata.extend_from_slice(
                                                    [ctx.signature.v].as_slice(),
                                                );
                                                display::show_status(true, true, ctx);
                                                send_data(comm, Ok(Some(rdata)));
                                            }
                                            false => {
                                                display::show_status(false, true, ctx);
                                                send_data(
                                                    comm,
                                                    Err(io::StatusWords::UserCancelled.into()),
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    send_data(comm, Ok(None));
                }
            }
            _ => {
                send_data(comm, Err(io::StatusWords::BadP1P2.into()));
            }
        },
        Ins::SignTxV1 => match p1 {
            0 => {
                ctx.reset();
                ctx.req_type = RequestType::SignTxV1;
                ctx.tx = Transaction::InvokeV1(InvokeTransactionV1::default());
                match crypto::set_derivation_path(&mut data, ctx) {
                    Ok(()) => {
                        send_data(comm, Ok(None));
                    }
                    Err(e) => {
                        send_data(comm, Err(e.into()));
                    }
                }
            }
            1 => {
                display::show_step(PARSING_STEP_TX_WORDING, ctx);
                transaction::set_tx_fields(data, &mut ctx.tx);
                send_data(comm, Ok(None));
            }
            2 => {
                #[cfg(any(target_os = "nanosplus", target_os = "nanox"))]
                display::show_step(PARSING_STEP_TX_WORDING, ctx);
                let nb_calls: u8 = FieldElement::from(data).into();
                transaction::set_calldata_nb(&mut ctx.tx, nb_calls);
                send_data(comm, Ok(None));
            }
            3 => {
                #[cfg(any(target_os = "nanosplus", target_os = "nanox"))]
                {
                    if p2 == SetCallStep::End.into() {
                        display::show_step(
                            format!(
                                "{}{}/{}",
                                PARSING_STEP_CALL_WORDING,
                                ctx.tx.get_nb_received_calls(),
                                ctx.tx.get_nb_calls()
                            )
                            .as_str(),
                            ctx,
                        );
                    }
                }
                #[cfg(any(target_os = "stax", target_os = "flex"))]
                {
                    if p2 == SetCallStep::New.into() {
                        display::show_step(
                            format!(
                                "{}{}/{}",
                                PARSING_STEP_CALL_WORDING,
                                ctx.tx.get_nb_received_calls() + 1,
                                ctx.tx.get_nb_calls()
                            )
                            .as_str(),
                            ctx,
                        );
                    }
                }
                if let Some(err) = transaction::set_calldata(data, p2.into(), &mut ctx.tx).err() {
                    send_data(comm, Err(Reply(err as u16)));
                }
                if p2 == transaction::SetCallStep::End.into() {
                    match transaction::tx_complete(&mut ctx.tx) {
                        None => {
                            send_data(comm, Ok(None));
                        }
                        Some(hash) => {
                            ctx.hash = hash;
                            match display::show_tx(ctx) {
                                Some(approved) => match approved {
                                    true => {
                                        rdata.extend_from_slice(ctx.hash.value.as_ref());
                                        crypto::sign_hash(ctx).unwrap();
                                        rdata.extend_from_slice([SIG_LENGTH].as_slice());
                                        rdata.extend_from_slice(ctx.signature.r.as_ref());
                                        rdata.extend_from_slice(ctx.signature.s.as_ref());
                                        rdata.extend_from_slice([ctx.signature.v].as_slice());
                                        display::show_status(true, true, ctx);
                                        send_data(comm, Ok(Some(rdata)));
                                    }
                                    false => {
                                        display::show_status(false, true, ctx);
                                        send_data(comm, Err(io::StatusWords::UserCancelled.into()));
                                    }
                                },
                                None => {
                                    let settings: Settings = Default::default();
                                    if settings.get_element(0) == 0 {
                                        display::blind_signing_enable_ui(ctx);
                                        send_data(comm, Err(io::StatusWords::UserCancelled.into()));
                                    } else {
                                        match display::show_hash(ctx, true) {
                                            true => {
                                                rdata.extend_from_slice(ctx.hash.value.as_ref());
                                                crypto::sign_hash(ctx).unwrap();
                                                rdata.extend_from_slice([SIG_LENGTH].as_slice());
                                                rdata.extend_from_slice(ctx.signature.r.as_ref());
                                                rdata.extend_from_slice(ctx.signature.s.as_ref());
                                                rdata.extend_from_slice(
                                                    [ctx.signature.v].as_slice(),
                                                );
                                                display::show_status(true, true, ctx);
                                                send_data(comm, Ok(Some(rdata)));
                                            }
                                            false => {
                                                display::show_status(false, true, ctx);
                                                send_data(
                                                    comm,
                                                    Err(io::StatusWords::UserCancelled.into()),
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    send_data(comm, Ok(None));
                }
            }
            _ => {
                send_data(comm, Err(io::StatusWords::BadP1P2.into()));
            }
        },
        Ins::SignDeployAccount => match p1 {
            0 => {
                ctx.reset();
                ctx.req_type = RequestType::SignDeployAccount;
                ctx.tx = Transaction::DeployAccountV3(DeployAccountTransactionV3::default());
                match crypto::set_derivation_path(&mut data, ctx) {
                    Ok(()) => {
                        send_data(comm, Ok(None));
                    }
                    Err(e) => {
                        send_data(comm, Err(e.into()));
                    }
                }
            }
            1 => {
                transaction::set_tx_fields(data, &mut ctx.tx);
                send_data(comm, Ok(None));
            }
            2 => {
                transaction::set_tx_fees(data, &mut ctx.tx);
                send_data(comm, Ok(None));
            }
            3 => {
                transaction::set_paymaster_data(data, p2, &mut ctx.tx);
                send_data(comm, Ok(None));
            }
            4 => {
                let constructor_calldata_length: u8 = FieldElement::from(data).into();
                transaction::set_calldata_nb(&mut ctx.tx, constructor_calldata_length);
                send_data(comm, Ok(None));
            }
            5 => {
                if let Some(err) = transaction::set_calldata(data, p2.into(), &mut ctx.tx).err() {
                    send_data(comm, Err(Reply(err as u16)));
                }
                match transaction::tx_complete(&mut ctx.tx) {
                    None => {
                        send_data(comm, Ok(None));
                    }
                    Some(hash) => {
                        ctx.hash = hash;
                        match display::show_tx(ctx) {
                            Some(approved) => match approved {
                                true => {
                                    rdata.extend_from_slice(ctx.hash.value.as_ref());
                                    crypto::sign_hash(ctx).unwrap();
                                    rdata.extend_from_slice([SIG_LENGTH].as_slice());
                                    rdata.extend_from_slice(ctx.signature.r.as_ref());
                                    rdata.extend_from_slice(ctx.signature.s.as_ref());
                                    rdata.extend_from_slice([ctx.signature.v].as_slice());
                                    display::show_status(true, true, ctx);
                                    send_data(comm, Ok(Some(rdata)));
                                }
                                false => {
                                    display::show_status(false, true, ctx);
                                    send_data(comm, Err(io::StatusWords::UserCancelled.into()));
                                }
                            },
                            None => {
                                send_data(comm, Err(io::StatusWords::UserCancelled.into()));
                            }
                        }
                    }
                }
            }
            _ => {
                send_data(comm, Err(io::StatusWords::BadP1P2.into()));
            }
        },
        Ins::SignDeployAccountV1 => match p1 {
            0 => {
                ctx.reset();
                ctx.req_type = RequestType::SignDeployAccountV1;
                ctx.tx = Transaction::DeployAccountV1(DeployAccountTransactionV1::default());
                match crypto::set_derivation_path(&mut data, ctx) {
                    Ok(()) => {
                        send_data(comm, Ok(None));
                    }
                    Err(e) => {
                        send_data(comm, Err(e.into()));
                    }
                }
            }
            1 => {
                transaction::set_tx_fields(data, &mut ctx.tx);
                send_data(comm, Ok(None));
            }
            2 => {
                transaction::set_tx_fees(data, &mut ctx.tx);
                send_data(comm, Ok(None));
            }
            3 => {
                let constructor_calldata_length: u8 = FieldElement::from(data).into();
                transaction::set_calldata_nb(&mut ctx.tx, constructor_calldata_length);
                send_data(comm, Ok(None));
            }
            4 => {
                if let Some(err) = transaction::set_calldata(data, p2.into(), &mut ctx.tx).err() {
                    send_data(comm, Err(Reply(err as u16)));
                }

                match transaction::tx_complete(&mut ctx.tx) {
                    None => {
                        send_data(comm, Ok(None));
                    }
                    Some(hash) => {
                        ctx.hash = hash;
                        match display::show_tx(ctx) {
                            Some(approved) => match approved {
                                true => {
                                    rdata.extend_from_slice(ctx.hash.value.as_ref());
                                    crypto::sign_hash(ctx).unwrap();
                                    rdata.extend_from_slice([SIG_LENGTH].as_slice());
                                    rdata.extend_from_slice(ctx.signature.r.as_ref());
                                    rdata.extend_from_slice(ctx.signature.s.as_ref());
                                    rdata.extend_from_slice([ctx.signature.v].as_slice());
                                    display::show_status(true, true, ctx);
                                    send_data(comm, Ok(Some(rdata)));
                                }
                                false => {
                                    display::show_status(false, true, ctx);
                                    send_data(comm, Err(io::StatusWords::UserCancelled.into()));
                                }
                            },
                            None => {
                                send_data(comm, Err(io::StatusWords::UserCancelled.into()));
                            }
                        }
                    }
                }
            }
            _ => {
                send_data(comm, Err(io::StatusWords::BadP1P2.into()));
            }
        },
        #[cfg(feature = "poseidon")]
        Ins::Poseidon => {
            let data = comm.get_data()?;
            let p1 = apdu_header.p1;

            match p1 {
                0 => {
                    let x = FieldElement::from(&data[0..32]);
                    let hash = crypto::poseidon::PoseidonStark252::hash_single(&x);
                    comm.append(hash.value.as_ref());
                }
                1 => {
                    let x = FieldElement::from(&data[0..32]);
                    let y = FieldElement::from(&data[32..64]);
                    let hash = crypto::poseidon::PoseidonStark252::hash(&x, &y);
                    comm.append(hash.value.as_ref());
                }
                2 => {
                    let a = FieldElement::from(data[0]);
                    let b = FieldElement::from(data[1]);
                    let c = FieldElement::from(data[2]);
                    let d = FieldElement::from(data[3]);
                    let e = FieldElement::from(data[4]);
                    let f = FieldElement::from(data[5]);

                    let values: [FieldElement; 6] = [a, b, c, d, e, f];
                    let hash = crypto::poseidon::PoseidonStark252::hash_many(&values);
                    comm.append(hash.value.as_ref());
                }
                3 => {
                    let a = FieldElement::from(data[0]);
                    let b = FieldElement::from(data[1]);
                    let c = FieldElement::from(data[2]);
                    let d = FieldElement::from(data[3]);
                    let e = FieldElement::from(data[4]);
                    let f = FieldElement::from(data[5]);
                    let mut hasher = crypto::poseidon::PoseidonHasher::default();
                    hasher.update(a);
                    hasher.update(b);
                    hasher.update(c);
                    hasher.update(d);
                    hasher.update(e);
                    hasher.update(f);
                    comm.append(hasher.finalize().value.as_ref());
                }
                _ => {
                    return Err(io::StatusWords::BadP1P2.into());
                }
            }
        }
    }
}

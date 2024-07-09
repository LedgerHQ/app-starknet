#![no_std]
#![no_main]

mod context;
mod crypto;
mod display;
mod types;

use context::{Ctx, RequestType};
use crypto::{get_pubkey, set_derivation_path, sign_hash};
use ledger_device_sdk::io;
use types::FieldElement;

ledger_device_sdk::set_panic!(ledger_device_sdk::exiting_panic);

#[cfg(any(target_os = "stax", target_os = "flex"))]
use ledger_device_sdk::nbgl::init_comm;

#[no_mangle]
extern "C" fn sample_main() {
    let mut comm = io::Comm::new();

    // Initialize reference to Comm instance for NBGL
    // API calls.
    #[cfg(any(target_os = "stax", target_os = "flex"))]
    init_comm(&mut comm);

    let mut ctx: Ctx = Ctx::new();

    loop {
        // Wait for either a specific button push to exit the app
        // or an APDU command
        if let io::Event::Command(ins) = display::main_ui(&mut comm) {
            match handle_apdu(&mut comm, ins, &mut ctx) {
                Ok(()) => comm.reply_ok(),
                Err(sw) => comm.reply(sw),
            }
        }
    }
}

#[repr(u8)]
enum Ins {
    GetVersion,
    GetPubkey { display: bool },
    SignHash,
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
            (2, _, _) => Ok(Ins::SignHash),
            (3, _, _) => Ok(Ins::Poseidon),
            (_, _, _) => Err(io::StatusWords::BadIns),
        }
    }
}

use ledger_device_sdk::io::Reply;

fn handle_apdu(comm: &mut io::Comm, ins: Ins, ctx: &mut Ctx) -> Result<(), Reply> {
    if comm.rx == 0 {
        return Err(io::StatusWords::NothingReceived.into());
    }

    let apdu_header = comm.get_apdu_metadata();
    if apdu_header.cla != 0x5A {
        return Err(io::StatusWords::BadCla.into());
    }

    match ins {
        Ins::GetVersion => {
            let version_major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u8>().unwrap();
            let version_minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u8>().unwrap();
            let version_patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u8>().unwrap();
            comm.append([version_major, version_minor, version_patch].as_slice());
        }
        Ins::GetPubkey { display } => {
            ctx.clear();
            ctx.req_type = RequestType::GetPubkey;

            let mut data = comm.get_data()?;

            let res = set_derivation_path(&mut data, ctx);
            match res {
                Err(e) => {
                    return Err(e.into());
                }
                Ok(()) => {
                    let pub_key = get_pubkey(ctx);
                    match pub_key {
                        Err(e) => {
                            return Err(Reply::from(e));
                        }
                        Ok(key) => {
                            let ret = match display {
                                false => true,
                                true => display::pkey_ui(key.as_ref()),
                            };
                            if ret {
                                comm.append(key.as_ref());
                            } else {
                                return Err(io::StatusWords::UserCancelled.into());
                            }
                        }
                    }
                }
            }
        }
        Ins::SignHash => {
            let p1 = apdu_header.p1;
            let mut data = comm.get_data()?;

            match p1 {
                0 => {
                    ctx.clear();
                    ctx.req_type = RequestType::SignHash;

                    set_derivation_path(&mut data, ctx)?;
                }
                _ => {
                    ctx.hash_info.m_hash = data.into();
                    match display::sign_ui(data) {
                        true => {
                            sign_hash(ctx).unwrap();
                        }
                        false => {
                            return Err(io::StatusWords::UserCancelled.into());
                        }
                    }
                    comm.append([0x41].as_slice());
                    comm.append(ctx.hash_info.r.as_ref());
                    comm.append(ctx.hash_info.s.as_ref());
                    comm.append([ctx.hash_info.v].as_slice());
                }
            }
        }
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
                    let mut hasher = crypto::poseidon::PoseidonHasher::new();
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
    Ok(())
}

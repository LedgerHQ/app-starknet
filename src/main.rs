#![no_std]
#![no_main]

mod context;
mod crypto;
mod display;

use crypto::{get_pubkey, set_derivation_path, sign_hash};

use context::{Ctx, RequestType};

use ledger_device_sdk::io;
use ledger_device_sdk::ui::gadgets::display_pending_review;

ledger_device_sdk::set_panic!(ledger_device_sdk::exiting_panic);

#[no_mangle]
extern "C" fn sample_main() {
    let mut comm = io::Comm::new();

    // Developer mode / pending review popup
    // must be cleared with user interaction
    display_pending_review(&mut comm);

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
            let p2 = apdu_header.p2;

            let mut data = comm.get_data()?;

            match p1 {
                0 => {
                    ctx.clear();
                    ctx.req_type = RequestType::SignHash;

                    set_derivation_path(&mut data, ctx)?;
                }
                _ => {
                    ctx.hash_info.m_hash = data.into();
                    if p2 > 0 {
                        match display::sign_ui(data) {
                            true => {
                                sign_hash(ctx).unwrap();
                            }
                            false => {
                                return Err(io::StatusWords::UserCancelled.into());
                            }
                        }
                    } else {
                        sign_hash(ctx).unwrap();
                    }
                    comm.append([0x41].as_slice());
                    comm.append(ctx.hash_info.r.as_ref());
                    comm.append(ctx.hash_info.s.as_ref());
                    comm.append([ctx.hash_info.v].as_slice());
                }
            }
        }
    }
    Ok(())
}

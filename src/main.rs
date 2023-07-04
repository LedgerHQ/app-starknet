#![no_std]
#![no_main]

mod crypto;
mod utils;
mod context;
mod display;
mod call;
use call::{
    handle_call_apdu, 
    CallInput
};
//mod transaction;

use crypto::{
    sign_hash, 
    pedersen, 
    get_pubkey, 
    set_derivation_path
};

use context::{Ctx, RequestType};
/*use transaction::{
    set_tx_fields,
    set_tx_calldata_lengths,
    set_tx_callarray,
    set_tx_calldata
};*/

use nanos_sdk::buttons::ButtonEvent;
use nanos_sdk::io;
use nanos_ui::ui;
use starknet_sdk::types::FieldElement;

nanos_sdk::set_panic!(nanos_sdk::exiting_panic);

#[no_mangle]
extern "C" fn sample_pending() {
    let mut comm = io::Comm::new();

    ui::SingleMessage::new("Pending").show();

    loop {
        match comm.next_event::<Ins>() {
            io::Event::Button(ButtonEvent::RightButtonRelease) => break,
            _ => (),
        }
    }
    ui::SingleMessage::new("Ledger review").show();
    loop {
        match comm.next_event::<Ins>() {
            io::Event::Button(ButtonEvent::BothButtonsRelease) => break,
            _ => (),
        }
    }
}

#[no_mangle]
extern "C" fn sample_main(_arg0: u32) {

    let mut comm = io::Comm::new();

    // Draw some 'welcome' screen
    ui::SingleMessage::new(display::WELCOME_SCREEN).show();

    let mut ctx: Ctx = Ctx::new();

    loop {        
        // Wait for either a specific button push to exit the app
        // or an APDU command
        match comm.next_event() {
            io::Event::Button(ButtonEvent::RightButtonRelease) => nanos_sdk::exit_app(0),        
            io::Event::Command(ins) => {
                match handle_apdu(&mut comm, ins, &mut ctx) {
                    Ok(()) => {
                        comm.reply_ok();
                    }
                    Err(sw) => comm.reply(sw),
                }
                ui::clear_screen();
                ui::SingleMessage::new(display::WELCOME_SCREEN).show();
            },
            _ => (),
        }
    }
}

#[repr(u8)]
enum Ins {
    GetVersion,
    GetPubkey,
    SignHash,
    SignTx,
    PedersenHash
}

impl TryFrom<io::ApduHeader> for Ins {
    type Error = ();
    fn try_from(header: io::ApduHeader) -> Result<Self, Self::Error> {
        match header.ins {
            0 => Ok(Ins::GetVersion),
            1 => Ok(Ins::GetPubkey),
            2 => Ok(Ins::SignHash),
            3 => Ok(Ins::SignTx),
            4 => Ok(Ins::PedersenHash),
            _ => Err(())
        }
    }
}

use nanos_sdk::io::Reply;

fn handle_apdu(comm: &mut io::Comm, ins: Ins, ctx: &mut Ctx) -> Result<(), Reply> {
    
    if comm.rx == 0 {
        return Err(io::StatusWords::NothingReceived.into());
    }
    
    let apdu_header = comm.get_apdu_metadata();
    if apdu_header.cla != 0x80 {
        return Err(io::StatusWords::BadCla.into());
    }

    match ins {
        Ins::GetVersion => {
            let version_major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u8>().unwrap();
            let version_minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u8>().unwrap();
            let version_patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u8>().unwrap();
            comm.append([version_major, version_minor, version_patch].as_slice());
        }
        Ins::GetPubkey => {

            ctx.clear();
            ctx.req_type = RequestType::GetPubkey;

            let mut data = comm.get_data()?;

            match set_derivation_path(&mut data, ctx) {
                Ok(()) => {
                    match get_pubkey(ctx) {
                        Ok(k) => {
                            comm.append(k.as_ref());
                        }
                        Err(e) => {
                            return Err(Reply::from(e));
                        } 
                    }
                }
                Err(e) => {
                    return Err(e.into());
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
                    ctx.hash = data.into();
                    if p2 > 0 {
                        match display::sign_ui(data) {
                            Ok(v) => {
                                if v {
                                    sign_hash(ctx).unwrap();
                                }
                                else {
                                    return Err(io::StatusWords::UserCancelled.into());
                                }
                            }
                            Err(_e) => {
                                return Err(io::SyscallError::Unspecified.into());
                            }
                        }
                    }
                    else {
                        sign_hash(ctx).unwrap();
                    }
                    comm.append([0x41].as_slice());
                    comm.append(ctx.signature.r.as_ref());
                    comm.append(ctx.signature.s.as_ref());
                    comm.append([ctx.signature.v].as_slice());
                }
            }
        }  
        Ins::PedersenHash => {
            ctx.clear();
            ctx.req_type = RequestType::ComputePedersen;
            let data = comm.get_data()?;
            let (a_s, b_s) = data.split_at(32);
            let mut a: FieldElement = a_s.into();
            let b: FieldElement = b_s.into();
            pedersen::pedersen_hash(&mut a, &b);
            comm.append(&a.value[..]);
        }
        Ins::SignTx => {

            let p1 = apdu_header.p1;
            let p2 = apdu_header.p2;
            let mut data = comm.get_data()?;

            match p1 {
                0 => {
                    ctx.clear();
                    ctx.req_type = RequestType::TestPlugin;
                    set_derivation_path(&mut data, ctx)?;
                }
                1 => {
                    ctx.tx_info.sender_address = data[0..32].into();
                    ctx.tx_info.max_fee = data[32..64].into();
                    ctx.tx_info.chain_id = data[64..96].into();
                    ctx.tx_info.nonce = data[96..128].into();
                    ctx.tx_info.version = data[128..160].into();
                    ctx.tx_info.callarray_len = data[160..192].into();
                }
                2 => {
                    let call_input: CallInput = p2.into();
                    ctx.is_first_loop = true;
                    let res = handle_call_apdu(data, ctx, call_input)?;
                    if !res {
                        return Err(io::StatusWords::UserCancelled.into());
                    }                        
                    if res && FieldElement::from(ctx.nb_call_rcv) == ctx.tx_info.callarray_len {
                        sign_hash(ctx).unwrap();
                        comm.append([0x41].as_slice());
                        comm.append(ctx.signature.r.as_ref());
                        comm.append(ctx.signature.s.as_ref());
                        comm.append([ctx.signature.v].as_slice());
                    }
                }
                3 => {
                    /* better multicall second loop */
                    let call_input: CallInput = p2.into();
                    ctx.is_first_loop = false;
                    handle_call_apdu(data, ctx, call_input)?;
                }
                _ => return Err(io::StatusWords::BadP1P2.into()),
            }
        }
    }
    Ok(())
}
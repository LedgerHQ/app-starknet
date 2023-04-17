#![no_std]
#![no_main]


mod crypto;
mod utils;
mod context;
mod display;
mod transaction;

use crypto::{
    sign_hash, 
    pedersen, 
    get_pubkey, 
    set_derivation_path
};

use context::{Ctx, RequestType, FieldElement};
use transaction::{
    set_tx_fields,
    set_tx_calldata_lengths,
    set_tx_callarray,
    set_tx_calldata
};

use nanos_sdk::buttons::ButtonEvent;
use nanos_sdk::io;
use nanos_ui::ui;

use nanos_sdk::bindings::{
    os_lib_call
};

use crate::utils::print::{printf};

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
extern "C" fn sample_main(arg0: u32) {

    let mut comm = io::Comm::new();

    // Draw some 'welcome' screen
    ui::SingleMessage::new(display::WELCOME_SCREEN).show();

    let mut ctx: Ctx = Ctx::new();

    loop {        
        // Wait for either a specific button push to exit the app
        // or an APDU command
        //printf("loop\n");
        match comm.next_event() {
            io::Event::Button(ButtonEvent::RightButtonRelease) => nanos_sdk::exit_app(0),        
            io::Event::Command(ins) => {
                match handle_apdu(&mut comm, ins, &mut ctx) {
                    Ok(()) => {
                        printf("Reply OK\n");
                        comm.reply_ok();
                        printf("Reply OK\n");
                    }
                    Err(sw) => comm.reply(sw),
                }
                printf("clear screen\n");
                ui::clear_screen();
                printf("display message\n");
                ui::SingleMessage::new(display::WELCOME_SCREEN).show();
                printf("message displayed\n");
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
    PedersenHash,
    SignTx,
    TestPlugin
}

impl TryFrom<u8> for Ins {
    type Error = ();
    fn try_from(ins: u8) -> Result<Self, Self::Error> {
        match ins {
            0 => Ok(Ins::GetVersion),
            1 => Ok(Ins::GetPubkey),
            2 => Ok(Ins::SignHash),
            3 => Ok(Ins::SignTx),
            4 => Ok(Ins::PedersenHash),
            5 => Ok(Ins::TestPlugin),
            _ => Err(())
        }
    }
}

use nanos_sdk::io::Reply;

fn handle_apdu(comm: &mut io::Comm, ins: Ins, ctx: &mut Ctx) -> Result<(), Reply> {
    
    if comm.rx == 0 {
        return Err(io::StatusWords::NothingReceived.into());
    }
    
    let (cla_byte, _) = comm.get_cla_ins();
    if cla_byte != 0x80 {
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

            let p1 = comm.get_p1();
            let p2 = comm.get_p2();

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
                    comm.append(ctx.hash_info.r.as_ref());
                    comm.append(ctx.hash_info.s.as_ref());
                    comm.append([ctx.hash_info.v].as_slice());
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
            
            let p1 = comm.get_p1();
            let p2 = comm.get_p2();
            let mut data = comm.get_data()?;

            match p1 {
                0 => {
                    ctx.clear();
                    ctx.req_type = RequestType::SignTransaction;
                    set_derivation_path(&mut data, ctx)?;
                }
                1 => {
                    set_tx_fields(&mut data, ctx);
                }
                2 => {
                    set_tx_calldata_lengths(&mut data, ctx);
                }
                3 => {
                    set_tx_callarray(&mut data, ctx, p2 as usize);
                }
                4 => {

                    match set_tx_calldata(data, ctx, p2 as usize) {
                        Ok(flag) => {
                            if !flag {
                                return Err(io::StatusWords::UserCancelled.into());
                            }
                        }
                        _ => ()
                    }

                    if p2 + 1 == ctx.tx_info.calldata.call_array_len.into() {
                        sign_hash(ctx).unwrap();
                        comm.append([65u8].as_slice());
                        comm.append(ctx.hash_info.r.as_ref());
                        comm.append(ctx.hash_info.s.as_ref());
                        comm.append([ctx.hash_info.v].as_slice());
                    }
                }
                _ => return Err(io::StatusWords::BadP1P2.into()),
            }
        }
        Ins::TestPlugin => {
            let plugin_name: &[u8] = "plugin-boilerplate\0".as_bytes();
            let mut arg: [u32; 3] = [0xFF; 3];
            arg[0] = plugin_name.as_ptr() as u32;
            unsafe {
                os_lib_call(arg.as_mut_ptr());
            }
            comm.append([0u8].as_slice());
            printf("Plugin has been called\n");
        }
    }
    printf("Returns OK\n");
    Ok(())
}
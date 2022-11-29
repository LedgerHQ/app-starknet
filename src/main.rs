#![no_std]
#![no_main]

mod crypto;
mod utils;
mod context;
mod display;

use crypto::{
    sign_hash, 
    pedersen, 
    get_pubkey, 
    get_derivation_path
};
use utils::print::printf;
use context::{Ctx, RequestType};

//use nanos_sdk::buttons::ButtonEvent;
use nanos_sdk::io;
use nanos_ui::ui;

nanos_sdk::set_panic!(nanos_sdk::exiting_panic);

#[no_mangle]
extern "C" fn sample_main() {
    let mut comm = io::Comm::new();

    // Draw some 'welcome' screen
    ui::SingleMessage::new(display::WELCOME_SCREEN).show();

    printf("Instantiate Ctx \n");
    let mut ctx: Ctx = Ctx::new();

    loop {        
        // Wait for either a specific button push to exit the app
        // or an APDU command
        match comm.next_event() {
            //io::Event::Button(ButtonEvent::RightButtonRelease) => nanos_sdk::exit_app(0),        
            io::Event::Command(ins) => {
                printf("event\n");
                match handle_apdu(&mut comm, ins, &mut ctx) {
                    Ok(()) => comm.reply_ok(),
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
    PedersenHash
}

impl TryFrom<u8> for Ins {
    type Error = ();
    fn try_from(ins: u8) -> Result<Self, Self::Error> {
        match ins {
            0 => Ok(Ins::GetVersion),
            1 => Ok(Ins::GetPubkey),
            2 => Ok(Ins::SignHash),
            3 => Ok(Ins::PedersenHash),
            _ => Err(())
        }
    }
}

use nanos_sdk::io::Reply;

fn handle_apdu(comm: &mut io::Comm, ins: Ins, ctx: &mut Ctx) -> Result<(), Reply> {
    
    printf("process APDU\n");

    if comm.rx == 0 {
        return Err(io::StatusWords::NothingReceived.into());
    }
    
    match ins {
        Ins::GetVersion => {
            let version_major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u8>().unwrap();
            let version_minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u8>().unwrap();
            let version_patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u8>().unwrap();
            comm.append([version_major, version_minor, version_patch].as_slice());
        }
        Ins::GetPubkey => {

            let mut data = comm.get_data()?;

            match get_derivation_path(&mut data, ctx.bip32_path.as_mut()) {
                Ok(()) => {
                    match get_pubkey(ctx.bip32_path.as_ref()) {
                        Ok(k) => {
                            comm.append(k.as_ref());
                        }
                        Err(e) => return Err(Reply::from(e)), 
                    }
                }
                Err(_e) => return Err(io::StatusWords::BadLen.into())
            }
        }
        Ins::SignHash => {

            ctx.req_type = RequestType::SignHash;

            let p1 = comm.get_p1();
            let p2 = comm.get_p2();

            let mut data = comm.get_data()?;

            match p1 {
                0 => {
                    get_derivation_path(&mut data, ctx.bip32_path.as_mut());
                }
                _ => {
                    ctx.hash_info.m_hash = data.into();
                    if p2 > 0 {
                        match display::sign_ui(data) {
                            Ok(v) => {
                                if v {
                                    sign_hash(ctx);
                                }
                            }
                            Err(_e) => (),
                        }
                    }
                    else {
                        sign_hash(ctx);
                    }
                    comm.append(ctx.hash_info.r.as_ref());
                    comm.append(ctx.hash_info.s.as_ref());
                }
            }
        }  
        Ins::PedersenHash => {
            printf("Compute Pedersen");
            ctx.req_type = RequestType::ComputePedersen;
            let data = comm.get_data()?;
            let (a, b) = data.split_at(32);
            let hash = pedersen::pedersen_hash(a, b);
            comm.append(&hash[..]);
        }
    }
    Ok(())
}

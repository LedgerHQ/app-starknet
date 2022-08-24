#![no_std]
#![no_main]

mod crypto_helpers;
mod utils;

use core::str::from_utf8;
use crypto_helpers::*;
use nanos_sdk::buttons::ButtonEvent;
use nanos_sdk::io;
use nanos_sdk::io::SyscallError;
use nanos_ui::ui;

nanos_sdk::set_panic!(nanos_sdk::exiting_panic);

/// Basic nested menu. Will be subject
/// to simplifications in the future.
#[allow(clippy::needless_borrow)]
fn menu_example() {
    loop {
        match ui::Menu::new(&[&"Infos", &"Back", &"Exit App"]).show() {
            0 => loop {
                match ui::Menu::new(&[&"Copyright", &"Authors", &"Back"]).show() {
                    0 => ui::popup("2020 Ledger"),
                    1 => ui::popup("???"),
                    _ => break,
                }
            },
            1 => return,
            2 => nanos_sdk::exit_app(0),
            _ => (),
        }
    }
}

/// This is the UI flow for signing, composed of a scroller
/// to read the incoming message, a panel that requests user
/// validation, and an exit message.
fn sign_ui(message: &[u8]) -> Result<Option<[u8; 72]>, SyscallError> {
    ui::popup("Message review");

    {
        let hex = utils::to_hex(message).map_err(|_| SyscallError::Overflow)?;
        let m = from_utf8(&hex).map_err(|_| SyscallError::InvalidParameter)?;

        ui::MessageScroller::new(m).event_loop();
    }

    if ui::Validator::new("Sign ?").ask() {
        let (sig, _sig_len) = detecdsa_sign(message).unwrap();
        //ui::popup("Done !");
        Ok(Some(sig))
    } else {
        //ui::popup("Cancelled");
        Ok(None)
    }
}

#[no_mangle]
extern "C" fn sample_main() {
    let mut comm = io::Comm::new();

    // Draw some 'welcome' screen
    ui::SingleMessage::new("S t a r k n e t").show();

    loop {
        
        // Wait for either a specific button push to exit the app
        // or an APDU command
        match comm.next_event() {
            io::Event::Button(ButtonEvent::RightButtonRelease) => nanos_sdk::exit_app(0),
            io::Event::Command(ins) => match handle_apdu(&mut comm, ins) {
                Ok(()) => comm.reply_ok(),
                Err(sw) => comm.reply(sw),
            },
            _ => (),
        }
    }
}

#[repr(u8)]
enum Ins {
    GetVersion,
    GetPubkey,
    Sign,
    Menu,
    Exit,
}

impl From<u8> for Ins {
    fn from(ins: u8) -> Ins {
        match ins {
            0 => Ins::GetVersion,
            1 => Ins::GetPubkey,
            2 => Ins::Sign,
            0xfe => Ins::Menu,
            0xff => Ins::Exit,
            _ => panic!(),
        }
    }
}

use nanos_sdk::io::Reply;

fn handle_apdu(comm: &mut io::Comm, ins: Ins) -> Result<(), Reply> {
    if comm.rx == 0 {
        return Err(io::StatusWords::NothingReceived.into());
    }

    match ins {
        Ins::GetVersion => {
            nanos_sdk::exit_app(0)
        }
        Ins::GetPubkey => {
            let data_path = comm.get_data()?;
            let mut path: [u32; 6] = [0u32; 6];
            get_derivation_path(data_path, &mut path[..]).unwrap();
            let pubkey = get_pubkey(&path[..]).unwrap();
            let key = pubkey.to_bytes();
            comm.append(&key)
        }
        Ins::Sign => {
            let out = sign_ui(comm.get_data()?)?;
            if let Some(o) = out {
                comm.append(&o)
            }
        }
        Ins::Menu => menu_example(),
        Ins::Exit => nanos_sdk::exit_app(0),
    }
    Ok(())
}

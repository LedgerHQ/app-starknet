#![no_std]
#![no_main]

mod context;
mod crypto;
mod display;

use crypto::{get_pubkey, set_derivation_path, sign_hash};

use context::{Ctx, RequestType};

use ledger_device_sdk::io;
use ledger_device_sdk::ui::bagls::*;
use ledger_device_sdk::ui::gadgets::*;
use ledger_device_sdk::ui::layout::Draw;
use ledger_device_sdk::ui::screen_util::*;
use ledger_device_sdk::uxapp::{UxEvent, BOLOS_UX_OK};

ledger_device_sdk::set_panic!(ledger_device_sdk::exiting_panic);

#[no_mangle]
extern "C" fn sample_main() {
    let mut comm = io::Comm::new();

    let mut ctx: Ctx = Ctx::new();

    let mut menu = &display::HOME_MENU;
    menu.pagelinks[0].page.place();
    LEFT_ARROW.display();
    RIGHT_ARROW.display();
    screen_update();

    let mut page_index: usize = 0;

    loop {
        // Wait for either a specific button push to exit the app
        // or an APDU command
        match comm.next_event() {
            io::Event::Button(evt) => {
                match evt {
                    ledger_device_sdk::buttons::ButtonEvent::BothButtonsRelease => {
                        let pl = menu.pagelinks[page_index];
                        if let Some(m) = pl.link {
                            menu = m;
                            page_index = 0;
                        }
                    }
                    ledger_device_sdk::buttons::ButtonEvent::LeftButtonRelease => {
                        if page_index as i16 - 1 < 0 {
                            page_index = menu.pagelinks.len() - 1;
                        } else {
                            page_index = page_index.saturating_sub(1);
                        }
                    }
                    ledger_device_sdk::buttons::ButtonEvent::RightButtonRelease => {
                        if page_index < menu.pagelinks.len() - 1 {
                            page_index += 1;
                        } else {
                            page_index = 0;
                        }
                    }
                    _ => (),
                }
                if menu.pagelinks.is_empty() {
                    // In the HELL menu
                    ledger_device_sdk::exit_app(0);
                }
                clear_screen();
                menu.pagelinks[page_index].page.place();
                LEFT_ARROW.display();
                RIGHT_ARROW.display();
                screen_update();
            }
            io::Event::Command(ins) => {
                match handle_apdu(&mut comm, ins, &mut ctx) {
                    Ok(()) => comm.reply_ok(),
                    Err(sw) => comm.reply(sw),
                }
                clear_screen();
                menu.pagelinks[page_index].page.place();
                LEFT_ARROW.display();
                RIGHT_ARROW.display();
                screen_update();
            }
            io::Event::Ticker => {
                // Pin lock management
                if UxEvent::Event.request() != BOLOS_UX_OK {
                    let (_res, ins) = UxEvent::block_and_get_event::<Ins>(&mut comm);
                    if let Some(_e) = ins {
                        comm.reply::<io::StatusWords>(io::StatusWords::Unknown);
                    }
                    // Redisplay screen
                    clear_screen();
                    menu.pagelinks[page_index].page.place();
                    LEFT_ARROW.display();
                    RIGHT_ARROW.display();
                    screen_update();
                }
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
    }
    Ok(())
}

#![no_std]
#![no_main]

mod crypto;
mod utils;
mod context;
mod display;
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

use heapless::{Vec, String};
use nanos_sdk::buttons::ButtonEvent;
use nanos_sdk::io;
use nanos_ui::ui;
use nanos_sdk::{
    string,
    testing
};
use nanos_sdk::starknet::{
    FieldElement, 
    TransactionInfo, 
    Call
};

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
    PedersenHash,
    //SignTx,
    TestPlugin
}

impl TryFrom<io::ApduHeader> for Ins {
    type Error = ();
    fn try_from(header: io::ApduHeader) -> Result<Self, Self::Error> {
        match header.ins {
            0 => Ok(Ins::GetVersion),
            1 => Ok(Ins::GetPubkey),
            2 => Ok(Ins::SignHash),
            //3 => Ok(Ins::SignTx),
            4 => Ok(Ins::PedersenHash),
            5 => Ok(Ins::TestPlugin),
            _ => Err(())
        }
    }
}

use nanos_sdk::io::Reply;
use nanos_sdk::plugin::{
    PluginCoreParams,
    PluginCheckParams,
    PluginInitParams,
    PluginFeedParams,
    PluginFinalizeParams,
    PluginQueryUiParams,
    PluginGetUiParams,
    PluginParams,
    plugin_call,
    PluginInteractionType,
    PluginResult
};

use nanos_sdk::starknet::{
    AbstractCall,
    AbstractCallData
};

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
        /*Ins::SignTx => {
            
            let p1 = apdu_header.p1;
            let p2 = apdu_header.p2;
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

                    if p2 + 1 == ctx.tx_info.calldata_v0.call_array_len.into() {
                        sign_hash(ctx).unwrap();
                        comm.append([65u8].as_slice());
                        comm.append(ctx.hash_info.r.as_ref());
                        comm.append(ctx.hash_info.s.as_ref());
                        comm.append([ctx.hash_info.v].as_slice());
                    }
                }
                _ => return Err(io::StatusWords::BadP1P2.into()),
            }
        }*/
        Ins::TestPlugin => {

            let p1 = apdu_header.p1;

            match p1 {
                0 => {

                    ctx.clear();
                    ctx.req_type = RequestType::TestPlugin;

                    // Hardcoded Tx 
                    {
                        ctx.tx_info.sender_address = FieldElement {
                            value: [
                                0x05, 0x12, 0xb3, 0xc8, 0xa1, 0x70, 0x42, 0xe5, 0x8a, 0xb4, 0x52, 0xa5, 0xec, 0x02, 0xe7, 0xba, 
                                0x94, 0x98, 0x72, 0xf5, 0xab, 0xd6, 0xb1, 0x8e, 0xc8, 0x3c, 0xf1, 0x86, 0x9a, 0x60, 0xfb, 0xe0
                            ]
                        };

                        ctx.call.to = FieldElement {
                            value: [
                                0x06, 0x8f, 0x5c, 0x6a, 0x61, 0x78, 0x07, 0x68, 0x45, 0x5d, 0xe6, 0x90, 0x77, 0xe0, 0x7e, 0x89, 
                                0x78, 0x78, 0x39, 0xbf, 0x81, 0x66, 0xde, 0xcf, 0xbf, 0x92, 0xb6, 0x45, 0x20, 0x9c, 0x0f, 0xb8
                            ]
                        };
                        ctx.call.method = String::from("transfer");
                        ctx.call.selector = FieldElement {
                            value: [
                                0x00, 0x83, 0xaf, 0xd3, 0xf4, 0xca, 0xed, 0xc6, 0xee, 0xbf, 0x44, 0x24, 0x6f, 0xe5, 0x4e, 0x38, 
                                0xc9, 0x5e, 0x31, 0x79, 0xa5, 0xec, 0x9e, 0xa8, 0x17, 0x40, 0xec, 0xa5, 0xb4, 0x82, 0xd1, 0x2e
                            ]
                        };
                        ctx.call.calldata.push(FieldElement::ZERO).unwrap();
                        ctx.call.calldata.push(FieldElement {
                            value: [
                                0x03, 0x5e, 0x4b, 0x54, 0x88, 0x1e, 0xdb, 0x79, 0xfb, 0x05, 0xac, 0x57, 0xf1, 0xd7, 0xb4, 0x5e, 
                                0x1b, 0x34, 0xb7, 0x10, 0x19, 0x00, 0x7f, 0xc1, 0x7b, 0x35, 0x9e, 0xf8, 0x04, 0x0f, 0xdb, 0x14
                            ]
                        }).unwrap();
                        ctx.call.calldata.push(FieldElement::ZERO).unwrap();
                        ctx.call.calldata.push(FieldElement {
                            value: [
                                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,   
                                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0xE8
                            ]
                        }).unwrap(); 
                    }

                    let mut plugin_bmc_params = PluginFeedParams {
                        core_params: Option::None,
                        data_in: &ctx.call as *const Call as *const u8,
                        data_out: &mut ctx.a_call as *mut AbstractCall as *mut u8,
                        result: PluginResult::Err
                    };
                    let plugin_params = PluginParams::Feed(&mut plugin_bmc_params);

                    testing::debug_print("=========================> Plugin call\n");
                    plugin_call("plugin-bmc\0", plugin_params, PluginInteractionType::Feed);
                    testing::debug_print("=========================> Plugin has been called\n");
                }
                1 => {

                    let mut plugin_check_params = PluginCheckParams {
                        core_params: Option::None,
                        data_in: core::ptr::null(),
                        data_in_len: 0,
                        result: PluginResult::Err
                    };
                    let plugin_params = PluginParams::Check(&mut plugin_check_params);

                    testing::debug_print("=========================> Plugin call\n");
                    plugin_call("plugin-erc20\0", plugin_params, PluginInteractionType::Check);
                    testing::debug_print("=========================> Plugin has been called\n");
                }
                2 => {
                    
                    let mut plugin_init_params = PluginInitParams {
                        core_params: Option::Some(PluginCoreParams {
                            plugin_internal_ctx: &mut ctx.plugin_internal_ctx as *mut u8,
                            plugin_internal_ctx_len: ctx.plugin_internal_ctx_len
                        }),
                        data_in: &ctx.a_call as *const AbstractCall as *const u8,
                        data_in_len: 0xFF,
                        result: PluginResult::Err
                    };

                    let plugin_params = PluginParams::Init(&mut plugin_init_params);

                    testing::debug_print("=========================> Plugin call\n");
                    plugin_call("plugin-erc20\0", plugin_params, PluginInteractionType::Init);
                    testing::debug_print("=========================> Plugin has been called\n");
                }
                3 => {

                    let mut plugin_feed_params = PluginFeedParams {
                        core_params: Option::Some(PluginCoreParams {
                            plugin_internal_ctx: &mut ctx.plugin_internal_ctx as *mut u8,
                            plugin_internal_ctx_len: ctx.plugin_internal_ctx_len
                        }),
                        data_in: &ctx.a_call.calldata as *const Vec<AbstractCallData, 8> as *const u8,
                        data_out: core::ptr::null_mut(),
                        result: PluginResult::Err
                    };

                    let plugin_params = PluginParams::Feed(&mut plugin_feed_params);

                    testing::debug_print("=========================> Plugin call\n");
                    plugin_call("plugin-erc20\0", plugin_params, PluginInteractionType::Feed);
                    testing::debug_print("=========================> Plugin has been called\n");

                }
                4 => {

                    let mut plugin_finalize_params = PluginFinalizeParams {
                        core_params: Option::Some(PluginCoreParams {
                            plugin_internal_ctx: &mut ctx.plugin_internal_ctx as *mut u8,
                            plugin_internal_ctx_len: ctx.plugin_internal_ctx_len
                        }),
                        num_ui_screens: 0,
                        result: PluginResult::Err
                    };

                    let plugin_params = PluginParams::Finalize(&mut plugin_finalize_params);

                    testing::debug_print("=========================> Plugin call\n");
                    plugin_call("plugin-erc20\0", plugin_params, PluginInteractionType::Finalize);
                    testing::debug_print("=========================> Plugin has been called\n");

                    testing::debug_print("Number of UI screens: ");
                    let s = string::to_utf8::<2>(string::Value::U8(plugin_finalize_params.num_ui_screens));
                    testing::debug_print(core::str::from_utf8(&s).unwrap());
                    testing::debug_print("\n");

                    ctx.num_ui_screens = plugin_finalize_params.num_ui_screens;
                }
                /*5 => {

                    let mut plugin_providedata_params = PluginProvideDataParams {
                        core_params: PluginCoreParams {
                            plugin_internal_ctx: &mut ctx.plugin_internal_ctx as *mut u8,
                            plugin_internal_ctx_len: ctx.plugin_internal_ctx_len,
                            app_data: core::ptr::null(),
                            app_data_len: 0,
                            plugin_result: PluginResult::Err
                        },
                        data: [0xFFu8; 128],
                        data_len: 15
                    };
                    
                    let plugin_params = PluginParams::ProvideData(&mut plugin_providedata_params);

                    testing::debug_print("=========================> Plugin call\n");
                    plugin_call("plugin-erc20\0", plugin_params, PluginInteractionType::ProvideData);
                    testing::debug_print("=========================> Plugin has been called\n");

                }*/
                6 => {

                    let mut plugin_queryui_params = PluginQueryUiParams {
                        core_params: Option::Some(PluginCoreParams {
                            plugin_internal_ctx: &mut ctx.plugin_internal_ctx as *mut u8,
                            plugin_internal_ctx_len: ctx.plugin_internal_ctx_len
                        }),
                        title: [0u8; 32],
                        title_len: 0,
                        result: PluginResult::Err
                    };

                    let plugin_params = PluginParams::QueryUi(&mut plugin_queryui_params);

                    testing::debug_print("=========================> Plugin call\n");
                    plugin_call("plugin-erc20\0", plugin_params, PluginInteractionType::QueryUi);
                    testing::debug_print("=========================> Plugin has been called\n");

                    ui::popup(core::str::from_utf8(&plugin_queryui_params.title[..plugin_queryui_params.title_len]).unwrap());
                }
                7 => {

                    let mut plugin_getui_params = PluginGetUiParams {
                        core_params: Option::Some(PluginCoreParams {
                            plugin_internal_ctx: &mut ctx.plugin_internal_ctx as *mut u8,
                            plugin_internal_ctx_len: ctx.plugin_internal_ctx_len,
                        }),
                        ui_screen_idx: 0,
                        title: [0u8; 32],
                        title_len: 0,
                        msg: [0u8; 64],
                        msg_len: 0,
                        result: PluginResult::Err
                     };

                    for i in 0..ctx.num_ui_screens {

                        plugin_getui_params.ui_screen_idx = i as usize;
                        let plugin_params = PluginParams::GetUi(&mut plugin_getui_params);

                        testing::debug_print("=========================> Plugin call\n");
                        plugin_call("plugin-erc20\0", plugin_params, PluginInteractionType::GetUi);
                        testing::debug_print("=========================> Plugin has been called\n");

                        let title = core::str::from_utf8(&plugin_getui_params.title[..plugin_getui_params.title_len]).unwrap();

                        match plugin_getui_params.msg_len {
                            0..=16 => {
                                let msg = core::str::from_utf8(&plugin_getui_params.msg[..plugin_getui_params.msg_len]).unwrap();
                                ui::MessageValidator::new(
                                    &[title, msg],
                                    &[&"Confirm"],
                                    &[&"Cancel"],
                                )
                                .ask();
                            },
                            17..=32 => {
                                let msg0 = core::str::from_utf8(&plugin_getui_params.msg[..16]).unwrap();
                                let msg1 = core::str::from_utf8(&plugin_getui_params.msg[17..plugin_getui_params.msg_len]).unwrap();
                                ui::MessageValidator::new(
                                    &[title, msg0, msg1],
                                    &[&"Confirm"],
                                    &[&"Cancel"],
                                )
                                .ask();
                            }
                            33..=64 => {
                                let msg0 = core::str::from_utf8(&plugin_getui_params.msg[..16]).unwrap();
                                let msg1 = core::str::from_utf8(&plugin_getui_params.msg[17..32]).unwrap();
                                let msg2 = core::str::from_utf8(&plugin_getui_params.msg[32..48]).unwrap();
                                let msg3 = core::str::from_utf8(&plugin_getui_params.msg[48..plugin_getui_params.msg_len]).unwrap();
                                ui::MessageValidator::new(
                                    &[title, msg0, msg1, msg2, msg3],
                                    &[&"Confirm"],
                                    &[&"Cancel"],
                                )
                                .ask();
                            }
                            _ => {
                            }
                        }
                    }
                }
                _ => return Err(io::StatusWords::BadP1P2.into()),
            }
            comm.append([0u8].as_slice());
        }
    }
    Ok(())
}
use starknet_sdk::types::{
    Call, FieldElement, AbstractCallData, AbstractCall
};
use nanos_sdk::io::{Reply};
use nanos_sdk::testing::debug_print;
use nanos_sdk::plugin::{
    PluginResult,
    PluginInteractionType,
    PluginParams,
    PluginCoreParams,
    PluginCheckParams,
    PluginFeedParams,
    PluginInitParams,
    PluginFinalizeParams,
    PluginGetUiParams,
    PluginQueryUiParams,
    plugin_call
};
use nanos_sdk::string;
use nanos_ui::ui;

use crate::context::Ctx;

pub enum CallInput {
    Full = 0x00,
    PartialStart = 0x01,
    PartialNext = 0x02,
    PartialEnd = 0x03,
    Unknown = 0xFF
}

impl From<CallInput> for u8 {
    fn from(c: CallInput) -> Self {
        match c {
            CallInput::Full => 0x00,
            CallInput::PartialStart => 0x01,
            CallInput::PartialNext => 0x02,
            CallInput::PartialEnd => 0x03,
            CallInput::Unknown => 0xFF
        }
    }
}

impl From<u8> for CallInput {
    fn from(b: u8) -> Self {
        match b {
            0 => CallInput::Full,
            1 => CallInput::PartialStart,
            2 => CallInput::PartialNext,
            3 => CallInput::PartialEnd,
            4.. => CallInput::Unknown
        }
    }
}

#[derive(Debug)]
pub enum CallError {
    Error = 0xCA00,
    TooManyDataError = 0xCA01,
}

impl From<CallError> for Reply {
    fn from(ce: CallError) -> Reply {
        Reply(ce as u16)
    }
}

pub fn handle_call_apdu(data: &[u8], ctx: &mut Ctx, step: CallInput) -> Result<(), CallError> {
    
    match step  {
        CallInput::Full => {        
            ctx.call.clear();
            save_call(data, &mut ctx.call);
            ctx.nb_calls_rcv += 1;
            if ctx.nb_calls_rcv == 1 && ctx.call.to == FieldElement::ZERO {
                {
                    let mut params = PluginCheckParams {
                        core_params: Option::None,
                        data_in: core::ptr::null(),
                        data_out: core::ptr::null_mut(),
                        result: PluginResult::Err
                    };
                    let plugin_params = PluginParams::Check(&mut params);

                    debug_print("=========================> Plugin call\n");
                    plugin_call("plugin-bmc\0", plugin_params, PluginInteractionType::Check);
                    debug_print("=========================> Plugin has been called\n");
                }
                ctx.is_bettermulticall = true;
            }
            else {
                process_call(ctx);
            } 
        }
        CallInput::PartialStart => {
            ctx.call.clear();
            save_call(data, &mut ctx.call);
        }
        CallInput::PartialNext => {
            append_calldata(data, &mut ctx.call)?;
        }
        CallInput::PartialEnd => {
            append_calldata(data, &mut ctx.call)?;
            ctx.nb_calls_rcv += 1;
            if ctx.nb_calls_rcv == 1 && ctx.call.to == FieldElement::ZERO {
                {
                    let mut params = PluginCheckParams {
                        core_params: Option::None,
                        data_in: core::ptr::null(),
                        data_out: core::ptr::null_mut(),
                        result: PluginResult::Err
                    };
                    let plugin_params = PluginParams::Check(&mut params);

                    debug_print("=========================> Plugin call\n");
                    plugin_call("plugin-bmc\0", plugin_params, PluginInteractionType::Check);
                    debug_print("=========================> Plugin has been called\n");
                }
                ctx.is_bettermulticall = true;
            }
            else {
                process_call(ctx);
            }
        }
        _ => ()
    }
    Ok(())
}

fn save_call(data: &[u8], call: &mut Call) {
    
    call.to = (&data[0..32]).into();
    call.selector = (&data[32..64]).into();

    let iter = (&data[64..]).chunks(32);

    for chunk in iter {
        call.calldata[call.calldata_len] = chunk.into();
        call.calldata_len += 1;
    }
    
}

fn append_calldata(data: &[u8], call: &mut Call) -> Result<(), CallError>{

    let iter = data.chunks(32);
    for chunk in iter {
        if call.calldata_len == 16 {
            return Err(CallError::TooManyDataError);
        }
        call.calldata[call.calldata_len] = chunk.into();
        call.calldata_len += 1;
    }
    Ok(())
}

fn process_call(ctx: &mut Ctx) {

    if ctx.is_first_loop {
        debug_print("===============================> FIRST LOOP\n");
    }
    else {
        debug_print("===============================> SECOND LOOP\n");
    }

    match ctx.is_bettermulticall {
        true => {
            debug_print("Better Multicall!\n");
            {
                let mut params = PluginFeedParams {
                    core_params: Option::None,
                    data_in: &ctx.call as *const Call as *const u8,
                    data_out: &mut ctx.a_call as *mut AbstractCall as *mut u8,
                    result: PluginResult::Err
                };
                let plugin_params = PluginParams::Feed(&mut params);
    
                debug_print("=========================> Plugin call\n");
                plugin_call("plugin-bmc\0", plugin_params, PluginInteractionType::Feed);
                debug_print("=========================> Plugin has been called\n");
            }

            match ctx.is_first_loop {
                true => {
                    for d in ctx.a_call.calldata.iter() {
                        match d {
                            AbstractCallData::CallRef(idx, shift) => {
                                ctx.call_to_nref[*idx] += 1;
                            }
                            _ => ()
                        }
                    }
                }
                false => {
                }
            }

        }
        false => {
            debug_print("Multicall!\n");
            {
                //ctx.a_call.copy_from(&ctx.call);
                ctx.a_call.to.copy_from(&ctx.call.to);
                ctx.a_call.method.copy_from(&ctx.call.method);
                ctx.a_call.selector.copy_from(&ctx.call.selector);
                for i in 0..ctx.call.calldata_len {
                    let mut fe = FieldElement::new();
                    fe.copy_from(&ctx.call.calldata[i]);
                    ctx.a_call.calldata[i] = AbstractCallData::Felt(fe);
                }
            }
            debug_print("Call to AbstractCall trivial conversion done!\n");
            /* Call Plugin */
            {
                /* FIND */

                /* CHECK */
                let mut plugin_check_params = PluginCheckParams {
                    core_params: Option::None,
                    data_in: core::ptr::null(),
                    data_out: core::ptr::null_mut(),
                    result: PluginResult::Err
                };
                let plugin_params = PluginParams::Check(&mut plugin_check_params);

                debug_print("=========================> Plugin call\n");
                plugin_call("plugin-erc20\0", plugin_params, PluginInteractionType::Check);
                debug_print("=========================> Plugin has been called\n");

                /* INIT */
                let mut plugin_init_params = PluginInitParams {
                    core_params: Option::Some(PluginCoreParams {
                        plugin_internal_ctx: &mut ctx.plugin_internal_ctx as *mut u8,
                        plugin_internal_ctx_len: ctx.plugin_internal_ctx_len
                    }),
                    data_in: &ctx.a_call as *const AbstractCall as *const u8,
                    data_out: core::ptr::null_mut(),
                    result: PluginResult::Err
                };

                let plugin_params = PluginParams::Init(&mut plugin_init_params);

                debug_print("=========================> Plugin call\n");
                plugin_call("plugin-erc20\0", plugin_params, PluginInteractionType::Init);
                debug_print("=========================> Plugin has been called\n");

                /* FEED */
                ctx.call_to_string[0] = string::String::try_from("grom.stark").unwrap();

                let mut plugin_feed_params = PluginFeedParams {
                    core_params: Option::Some(PluginCoreParams {
                        plugin_internal_ctx: &mut ctx.plugin_internal_ctx as *mut u8,
                        plugin_internal_ctx_len: ctx.plugin_internal_ctx_len
                    }),
                    data_in: &(&ctx.a_call.calldata, &ctx.call_to_string) as *const (&[AbstractCallData; 8], &[string::String<32>; 16]) as *const u8,
                    data_out: core::ptr::null_mut(),
                    result: PluginResult::Err
                };

                let plugin_params = PluginParams::Feed(&mut plugin_feed_params);

                debug_print("=========================> Plugin call\n");
                plugin_call("plugin-erc20\0", plugin_params, PluginInteractionType::Feed);
                debug_print("=========================> Plugin has been called\n");

                /* FINALIZE */
                let mut plugin_finalize_params = PluginFinalizeParams {
                    core_params: Option::Some(PluginCoreParams {
                        plugin_internal_ctx: &mut ctx.plugin_internal_ctx as *mut u8,
                        plugin_internal_ctx_len: ctx.plugin_internal_ctx_len
                    }),
                    num_ui_screens: 0,
                    data_to_display: string::String::<64>::new(),
                    result: PluginResult::Err
                };

                let plugin_params = PluginParams::Finalize(&mut plugin_finalize_params);

                debug_print("=========================> Plugin call\n");
                plugin_call("plugin-erc20\0", plugin_params, PluginInteractionType::Finalize);
                debug_print("=========================> Plugin has been called\n");

                debug_print("Number of UI screens: ");
                let s: string::String::<2> = plugin_finalize_params.num_ui_screens.into();
                debug_print(s.as_str());
                debug_print("\n");

                ctx.num_ui_screens = plugin_finalize_params.num_ui_screens;

                /* QUERY UI */
                let mut plugin_queryui_params = PluginQueryUiParams {
                    core_params: Option::Some(PluginCoreParams {
                        plugin_internal_ctx: &mut ctx.plugin_internal_ctx as *mut u8,
                        plugin_internal_ctx_len: ctx.plugin_internal_ctx_len
                    }),
                    title: string::String::<32>::new(),
                    result: PluginResult::Err
                };

                let plugin_params = PluginParams::QueryUi(&mut plugin_queryui_params);

                debug_print("=========================> Plugin call\n");
                plugin_call("plugin-erc20\0", plugin_params, PluginInteractionType::QueryUi);
                debug_print("=========================> Plugin has been called\n");

                ui::popup(plugin_queryui_params.title.as_str());

                /* GET UI */
                let mut plugin_getui_params = PluginGetUiParams {
                    core_params: Option::Some(PluginCoreParams {
                        plugin_internal_ctx: &mut ctx.plugin_internal_ctx as *mut u8,
                        plugin_internal_ctx_len: ctx.plugin_internal_ctx_len,
                    }),
                    ui_screen_idx: 0,
                    title: string::String::<32>::new(),
                    msg: string::String::<64>::new(),
                    result: PluginResult::Err
                 };

                for i in 0..ctx.num_ui_screens {

                    plugin_getui_params.ui_screen_idx = i as usize;
                    let plugin_params = PluginParams::GetUi(&mut plugin_getui_params);

                    debug_print("=========================> Plugin call\n");
                    plugin_call("plugin-erc20\0", plugin_params, PluginInteractionType::GetUi);
                    debug_print("=========================> Plugin has been called\n");

                    let title = plugin_getui_params.title.as_str();
                    debug_print(title);
                    debug_print("\n");
                    debug_print(plugin_getui_params.msg.as_str());
                    debug_print("\n");

                    match plugin_getui_params.msg.len {
                        0..=16 => {
                            let msg = plugin_getui_params.msg.as_str();
                            ui::MessageValidator::new(
                                &[title, msg],
                                &[&"Confirm"],
                                &[&"Cancel"],
                            )
                            .ask();
                        },
                        17..=32 => {
                            let s = plugin_getui_params.msg.as_str();
                            let msg0 = &s[..16];
                            let msg1 = &s[16..plugin_getui_params.msg.len];
                            ui::MessageValidator::new(
                                &[title, msg0, msg1],
                                &[&"Confirm"],
                                &[&"Cancel"],
                            )
                            .ask();
                        }
                        33..=64 => {
                            let s = plugin_getui_params.msg.as_str();
                            let msg0 = &s[..16];
                            let msg1 = &s[16..32];
                            let msg2 = &s[32..48];
                            let msg3 = &s[48..plugin_getui_params.msg.len];
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
        }
    }

    if !ctx.is_first_loop {

        debug_print("to: ");
        debug_print(string::String::<64>::from(&ctx.a_call.to).as_str());
        debug_print("\n");

        debug_print("selector: ");
        debug_print(string::String::<64>::from(&ctx.a_call.selector).as_str());
        debug_print("\n");
        
        for i in 0..ctx.a_call.calldata_len {
            debug_print("calldata: ");
            match ctx.a_call.calldata[i] {
                AbstractCallData::Felt(f) => {
                    debug_print(string::String::<64>::from(&f).as_str());
                }
                _ => debug_print("Ref or CallRef")
            }
            debug_print("\n");
        }
    }
}
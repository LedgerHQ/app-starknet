use starknet_sdk::types::{
    Call, FieldElement, AbstractCallData, AbstractCall, UiParam, TransactionInfo
};
use nanos_sdk::io::{Reply};
use nanos_sdk::testing::debug_print;
use nanos_sdk::plugin::{
    PluginResult,
    PluginInteractionType,
    PluginParam,
    plugin_call
};
use nanos_sdk::string;
use nanos_ui::ui;

use crate::context::Ctx;
use crate::crypto::pedersen::{
    pedersen_hash,
    pedersen_shift
};

mod plugin;
use plugin::*;

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

pub fn handle_call_apdu(data: &[u8], ctx: &mut Ctx, step: CallInput) -> Result<bool, CallError> {
    match step  {
        CallInput::Full => {        
            ctx.call.clear();
            save_call(data, &mut ctx.call);
            Ok(process_call(ctx))
        }
        CallInput::PartialStart => {
            ctx.call.clear();
            save_call(data, &mut ctx.call);
            Ok(true)
        }
        CallInput::PartialNext => {
            append_calldata(data, &mut ctx.call)?;
            Ok(true)
        }
        CallInput::PartialEnd => {
            append_calldata(data, &mut ctx.call)?;
            Ok(process_call(ctx))
        }
        _ => Ok(false)
    }
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

fn process_call(ctx: &mut Ctx) -> bool {

    if ctx.nb_call_rcv == 0 {

        // Init Pedersen Hash
        pedersen_hash(&mut ctx.hash, &ctx.tx_info.callarray_len);
        ctx.nb_hash = 1;

        if ctx.call.to == FieldElement::ZERO {

            // Update Pedersen Hash
            update_pedersen(&ctx.call, &mut ctx.hash);
            ctx.nb_hash += 3 + ctx.call.calldata_len;

            // To do: check BMC plugin
            ctx.is_bettermulticall = true;
            ctx.nb_call_rcv += 1;
            return true;
        }
    }

    // Convert Call -> ACall (BMC or Trivial)
    if ctx.is_bettermulticall {
        let mut params = PluginParam {
            plugin_internal_ctx: core::ptr::null_mut(),
            plugin_internal_ctx_len: 0,
            data_in: &ctx.call as *const Call as *const u8,
            data_out: &mut ctx.a_call as *mut AbstractCall as *mut u8,
            result: PluginResult::Err
        };
        plugin_call("plugin-bmc\0", &mut params, PluginInteractionType::Feed);
    }
    else {
        ctx.a_call.copy_from(&ctx.call);
    }
    
    // Find and check plugin (todo: manage "plugin not found" case)
    let plugin_name = plugin_find(ctx).unwrap();
    
    // Plugin cycle
    {
        /* INIT */
        let mut params = PluginParam {
            plugin_internal_ctx: &mut ctx.plugin_internal_ctx as *mut u8,
            plugin_internal_ctx_len: ctx.plugin_internal_ctx.len(),
            data_in: &ctx.a_call as *const AbstractCall as *const u8,
            data_out: core::ptr::null_mut(),
            result: PluginResult::Err
        };
        plugin_call(plugin_name, &mut params, PluginInteractionType::Init);

        /* FEED */
        let mut params = PluginParam {
            plugin_internal_ctx: &mut ctx.plugin_internal_ctx as *mut u8,
            plugin_internal_ctx_len: ctx.plugin_internal_ctx.len(),
            data_in: &(&ctx.a_call.calldata, &ctx.call_to_string) as *const (&[AbstractCallData; 8], &[string::String<64>; 8]) as *const u8,
            data_out: core::ptr::null_mut(),
            result: PluginResult::Err
        };
        plugin_call(plugin_name, &mut params, PluginInteractionType::Feed);

        /* FINALIZE */
        let mut ui: UiParam = Default::default();
        let mut params = PluginParam {
            plugin_internal_ctx: &mut ctx.plugin_internal_ctx as *mut u8,
            plugin_internal_ctx_len: ctx.plugin_internal_ctx.len(),
            data_in: core::ptr::null(),
            data_out: &mut ui as *mut UiParam as *mut u8,
            result: PluginResult::Err
        };
        plugin_call(plugin_name, &mut params, PluginInteractionType::Finalize);

        ctx.num_ui_screens = ui.num_ui_screens;
        ctx.call_to_string[ctx.nb_call_rcv].copy_from(&ui.msg);
        
         // Update Pedersen Hash
         update_pedersen(&ctx.call, &mut ctx.hash);
         ctx.nb_hash += 3 + ctx.call.calldata_len;

        ctx.nb_call_rcv += 1;

        if FieldElement::from(ctx.nb_call_rcv) == ctx.tx_info.callarray_len {
            finalize_pedersen(&ctx.tx_info, &mut ctx.hash, ctx.nb_hash);
        }

        if ctx.num_ui_screens == 0 {
            true
        }
        else {

            let mut acknowledged: bool = false;
        
            /* QUERY UI */
            let mut params = PluginParam {
                plugin_internal_ctx: &mut ctx.plugin_internal_ctx as *mut u8,
                plugin_internal_ctx_len: ctx.plugin_internal_ctx.len(),
                data_in: core::ptr::null(),
                data_out: &mut ui.title as *mut string::String<32> as *mut u8,
                result: PluginResult::Err
            };
            plugin_call(plugin_name, &mut params, PluginInteractionType::QueryUi);

            ui::popup(ui.title.as_str());

            /* GET UI */
            for ui_screen_idx in 0..ctx.num_ui_screens {
                
                let mut params = PluginParam {
                    plugin_internal_ctx: &mut ctx.plugin_internal_ctx as *mut u8,
                    plugin_internal_ctx_len: ctx.plugin_internal_ctx.len(),
                    data_in: &ui_screen_idx as *const u8,
                    data_out: &mut ui as *mut UiParam as *mut u8,
                    result: PluginResult::Err
                };
                plugin_call(plugin_name, &mut params, PluginInteractionType::GetUi);

                debug_print(ui.title.as_str());
                debug_print("\n");
                debug_print(ui.msg.as_str());
                debug_print("\n");

                match ui.msg.len {
                    0..=16 => {
                        acknowledged = ui::MessageValidator::new(
                            &[ui.title.as_str(), ui.msg.as_str()],
                            &[&"Confirm"],
                            &[&"Cancel"]).ask();
                    },
                    17..=32 => {
                        let s = ui.msg.as_str();
                        let msg0 = &s[..16];
                        let msg1 = &s[16..ui.msg.len];
                        acknowledged = ui::MessageValidator::new(
                            &[ui.title.as_str(), msg0, msg1],
                            &[&"Confirm"],
                            &[&"Cancel"],
                        )
                        .ask();
                    }
                    33..=64 => {
                        let s = ui.msg.as_str();
                        let msg0 = &s[..16];
                        let msg1 = &s[16..32];
                        let msg2 = &s[32..48];
                        let msg3 = &s[48..ui.msg.len];
                        acknowledged = ui::MessageValidator::new(
                            &[ui.title.as_str(), msg0, msg1, msg2, msg3],
                            &[&"Confirm"],
                            &[&"Cancel"],
                        )
                        .ask();
                    }
                    _ => {
                        acknowledged = false;
                    }
                }
            }
            acknowledged
        }
    }
}

fn update_pedersen(call: &Call, hash: &mut FieldElement) {
    pedersen_hash(hash, &call.to);
    pedersen_hash(hash, &call.selector);
    pedersen_hash(hash, &FieldElement::from(call.calldata_len));
    for i in 0..call.calldata_len {
        pedersen_hash(hash, &call.calldata[i]);
    }
}

fn finalize_pedersen(tx_info: &TransactionInfo, hash: &mut FieldElement, nb_hash: usize) {
    
    let mut pedersen: FieldElement = Default::default();

    // do not forget to finalize Pedersen hash of calldata
    // see https://docs.starknet.io/documentation/architecture_and_concepts/Hashing/hash-functions/#pedersen_array_hash
    let mut n: FieldElement = FieldElement::from(nb_hash); 
    pedersen_hash(hash, &n);

    pedersen_hash(&mut pedersen, &FieldElement::INVOKE);
    pedersen_hash(&mut pedersen, &tx_info.version);
    pedersen_hash(&mut pedersen, &tx_info.sender_address);
    pedersen_hash(&mut pedersen, &FieldElement::ZERO);
    pedersen_hash(&mut pedersen, hash);
    pedersen_hash(&mut pedersen, &tx_info.max_fee);
    pedersen_hash(&mut pedersen, &tx_info.chain_id);
    pedersen_hash(&mut pedersen, &tx_info.nonce);
    n = 8u8.into();
    pedersen_hash(&mut pedersen, &n);

    pedersen_shift(&mut pedersen);

    hash.copy_from(&pedersen);
}
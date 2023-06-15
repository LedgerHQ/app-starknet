use starknet_sdk::types::{
    Call, FieldElement, AbstractCallData, AbstractCall
};

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

pub fn plugin_check(ctx: &mut Ctx, plugin_name: &str, params: &mut PluginCheckParams){

    let plugin_params = PluginParams::Check(params);

    debug_print("=========================> Plugin call\n");
    plugin_call(plugin_name, plugin_params, PluginInteractionType::Check);
    debug_print("=========================> Plugin has been called\n");
}

pub fn plugin_init(ctx: &mut Ctx, plugin_name: &str, params: &mut PluginInitParams){

    let plugin_params = PluginParams::Init(params);

    debug_print("=========================> Plugin call\n");
    plugin_call(plugin_name, plugin_params, PluginInteractionType::Init);
    debug_print("=========================> Plugin has been called\n");
}

pub fn plugin_feed(ctx: &mut Ctx, plugin_name: &str, params: &mut PluginFeedParams){
    
    //ctx.call_to_string[0] = string::String::try_from("grom.stark").unwrap();

    let plugin_params = PluginParams::Feed(params);

    debug_print("=========================> Plugin call\n");
    plugin_call(plugin_name, plugin_params, PluginInteractionType::Feed);
    debug_print("=========================> Plugin has been called\n");
}

pub fn plugin_finalize(ctx: &mut Ctx, plugin_name: &str, params: &mut PluginFinalizeParams){

    let plugin_params = PluginParams::Finalize(params);

    debug_print("=========================> Plugin call\n");
    plugin_call(plugin_name, plugin_params, PluginInteractionType::Finalize);
    debug_print("=========================> Plugin has been called\n");

    debug_print("Number of UI screens: ");
    let s: string::String::<2> = params.num_ui_screens.into();
    debug_print(s.as_str());
    debug_print("\n");
}

pub fn plugin_queryui(ctx: &mut Ctx, plugin_name: &str, params: &mut PluginQueryUiParams){

    let plugin_params = PluginParams::QueryUi(params);

    debug_print("=========================> Plugin call\n");
    plugin_call(plugin_name, plugin_params, PluginInteractionType::QueryUi);
    debug_print("=========================> Plugin has been called\n");
}

pub fn plugin_getui(ctx: &mut Ctx, plugin_name: &str, params: &mut PluginGetUiParams){
   
    let plugin_params = PluginParams::GetUi(params);

    debug_print("=========================> Plugin call\n");
    plugin_call(plugin_name, plugin_params, PluginInteractionType::GetUi);
    debug_print("=========================> Plugin has been called\n");
}
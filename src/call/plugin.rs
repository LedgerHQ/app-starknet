use starknet_sdk::types::FieldElement;

use nanos_sdk::plugin::{
    PluginResult,
    PluginInteractionType,
    PluginParam,
    plugin_call
};

use crate::context::Ctx;

struct PluginItem<'a> {
    addr: FieldElement,
    name: &'a str,
}

pub fn plugin_find(ctx: &mut Ctx) -> Option<&'static str> {
    let plugins: [PluginItem; 3] = [
        PluginItem {
            addr: FieldElement {
                value: [
                    0x04, 0x9d, 0x36, 0x57, 0x0d, 0x4e, 0x46, 0xf4, 0x8e, 0x99, 0x67, 0x4b, 0xd3,
                    0xfc, 0xc8, 0x46, 0x44, 0xdd, 0xd6, 0xb9, 0x6f, 0x7c, 0x74, 0x1b, 0x15, 0x62,
                    0xb8, 0x2f, 0x9e, 0x00, 0x4d, 0xc7,
                ],
            },
            name: "plugin-erc20\0",
        },
        PluginItem {
            addr: FieldElement {
                value: [
                    0x06, 0x8f, 0x5c, 0x6a, 0x61, 0x78, 0x07, 0x68, 0x45, 0x5d, 0xe6, 0x90, 0x77,
                    0xe0, 0x7e, 0x89, 0x78, 0x78, 0x39, 0xbf, 0x81, 0x66, 0xde, 0xcf, 0xbf, 0x92,
                    0xb6, 0x45, 0x20, 0x9c, 0x0f, 0xb8,
                ],
            },
            name: "plugin-erc20\0",
        },
        PluginItem {
            addr: FieldElement {
                value: [
                    0x00, 0x3b, 0xab, 0x26, 0x8e, 0x93, 0x2d, 0x2c, 0xec, 0xd1, 0x94, 0x6f, 0x10,
                    0x0a, 0xe6, 0x7c, 0xe3, 0xdf, 0xf9, 0xfd, 0x23, 0x41, 0x19, 0xea, 0x2f, 0x6d,
                    0xa5, 0x7d, 0x16, 0xd2, 0x9f, 0xce,
                ],
            },
            name: "plugin-starknetid\0",
        },
    ];

    let mut plugin_name = "";
    for item in plugins {
        if ctx.call.to == item.addr {
            plugin_name = item.name;
            break;
        }
    }

    /* CHECK */
    let mut params = PluginParam {
        plugin_internal_ctx: core::ptr::null_mut(),
        plugin_internal_ctx_len: 0,
        data_in: core::ptr::null(),
        data_out: core::ptr::null_mut(),
        result: PluginResult::Err,
    };
    plugin_call(plugin_name, &mut params, PluginInteractionType::Check);
    Some(plugin_name)
}

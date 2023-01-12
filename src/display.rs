use nanos_ui::ui;
use nanos_sdk::io;

use crate::{utils, context};

pub const WELCOME_SCREEN: &str = "S T A R K N E T";

/// This is the UI flow for signing, composed of a scroller
/// to read the incoming message, a panel that requests user
/// validation, and an exit message.
pub fn sign_ui(message: &[u8]) -> Result<bool, io::SyscallError> {

    ui::popup("Tx hash review:");
    {
        let hex: [u8; 64] = utils::to_hex(message).map_err(|_| io::SyscallError::Overflow)?;
        let m = core::str::from_utf8(&hex).map_err(|_| io::SyscallError::InvalidParameter)?;
        ui::MessageScroller::new(m).event_loop();
    }

    match ui::Validator::new("Sign?").ask() {
        true => Ok(true),
        false => Ok(false)
    }
}

pub fn sign_tx_ui(tx: &context::Transaction, n: usize, calldata: &[u8]) -> Result<bool, ()> {
    
    let mut hex: [u8; 64];
    let mut m: &str;

    if n == 0 {
        ui::popup("Tx review");
    
        ui::popup("Account:");
        hex = utils::to_hex(&tx.sender_address.value[..]).unwrap();
        m = core::str::from_utf8(&hex).unwrap();
        ui::MessageScroller::new(m).event_loop();
        
        ui::popup("MaxFee:");
        hex = utils::to_hex(&tx.max_fee.value[..]).unwrap();
        m = core::str::from_utf8(&hex).unwrap();
        ui::MessageScroller::new(m).event_loop();
    
        ui::popup("Nonce:");
        hex = utils::to_hex(&tx.nonce.value[..]).unwrap();
        m = core::str::from_utf8(&hex).unwrap();
        ui::MessageScroller::new(m).event_loop();
    }

    ui::popup("Calldata:");
    ui::popup("Contract:");
    hex = utils::to_hex(&tx.calldata.calls[n].to.value[..]).unwrap();
    m = core::str::from_utf8(&hex).unwrap();
    ui::MessageScroller::new(m).event_loop();
    ui::popup("Selector:");
    m = core::str::from_utf8(&tx.calldata.calls[n].entry_point[0..tx.calldata.calls[n].entry_point_length as usize]).unwrap();
    ui::MessageScroller::new(m).event_loop();

    let mut s_start: usize;
    let mut s_end: usize;
    let mut s: &[u8];
    let data_len: u8 = tx.calldata.calls[n].data_len.into();
    ui::popup("calldata:");
    for i in 0..data_len {
        s_start = (i * 32).into();
        s_end = s_start + 32;
        s = &calldata[s_start..s_end];
        hex = utils::to_hex(s).unwrap();
        m = core::str::from_utf8(&hex).unwrap();
        ui::MessageScroller::new(m).event_loop();
    }
    
    match ui::Validator::new("Approve?").ask() {
        true => Ok(true),
        false => Ok(false)
    }
}
use nanos_ui::ui;
use nanos_sdk::io;
use nanos_sdk::starknet::Transaction;

use crate::{ utils };

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

pub fn sign_tx_ui(tx: &Transaction, n: usize, calldata: &[u8]) -> Result<bool, ()> {
    
    let mut hex: [u8; 64];
    let mut m: &str;

    if n == 0 {
        if tx.calldata.len() > 1 {
            ui::popup("Review Multicall Tx");
        }
        else {
            ui::popup("Review Tx");
        }

        hex = utils::to_hex(&tx.tx_info.sender_address.value[..]).unwrap();
        m = core::str::from_utf8(&hex).unwrap();
        if !ui::MessageValidator::new(
            &[&"Account:", &m[0..16],&m[16..32], &m[32..48], &m[48..64]],
            &[&"Confirm"],
            &[&"Cancel"],
        )
        .ask() {
            return Ok(false);
        }
        
        hex = utils::to_hex(&tx.tx_info.max_fee.value[..]).unwrap();
        m = core::str::from_utf8(&hex).unwrap();
        if !ui::MessageValidator::new(
            &[&"MaxFee:", &m[0..16],&m[16..32], &m[32..48], &m[48..64]],
            &[&"Confirm"],
            &[&"Cancel"],
        )
        .ask() {
            return Ok(false);
        }

        hex = utils::to_hex(&tx.tx_info.nonce.value[..]).unwrap();
        m = core::str::from_utf8(&hex).unwrap();
        if !ui::MessageValidator::new(
            &[&"Nonce:", &m[0..16],&m[16..32], &m[32..48], &m[48..64]],
            &[&"Confirm"],
            &[&"Cancel"],
        )
        .ask() {
            return Ok(false);
        }
    }

    if tx.calldata.len() > 1 {
        ui::popup("Review Tx Multicalldata:");
    }
    else {
        ui::popup("Review Tx Calldata:");
    }

    hex = utils::to_hex(&tx.calldata[n].to.value[..]).unwrap();
    m = core::str::from_utf8(&hex).unwrap();
    if !ui::MessageValidator::new(
        &[&"Contract:", &m[0..16],&m[16..32], &m[32..48], &m[48..64]],
        &[&"Confirm"],
        &[&"Cancel"],
    )
    .ask() {
        return Ok(false);
    }

    //m = core::str::from_utf8(&tx.calldata_v0.calls[n].entry_point[0..tx.calldata_v0.calls[n].entry_point_length as usize]).unwrap();
    if !ui::MessageValidator::new(
        &[&"Selector:", tx.calldata[n].method.as_str()],
        &[&"Confirm"],
        &[&"Cancel"],
    )
    .ask() {
        return Ok(false);
    }

    /*let mut s_start: usize;
    let mut s_end: usize;
    let mut s: &[u8];
    let data_len: u8 = tx.calldata_v0.calls[n].data_len.into();
    for i in 0..data_len {
        s_start = (i * 32).into();
        s_end = s_start + 32;
        s = &calldata[s_start..s_end];
        hex = utils::to_hex(s).unwrap();
        m = core::str::from_utf8(&hex).unwrap();
        if !ui::MessageValidator::new(
            &[&"Selector calldata:", &m[0..16],&m[16..32], &m[32..48], &m[48..64]],
            &[&"Confirm"],
            &[&"Cancel"],
        )
        .ask() {
            return Ok(false);
        }
    }*/
    for d in &tx.calldata[n].calldata {
        hex = utils::to_hex(d.value.as_slice()).unwrap();
        m = core::str::from_utf8(&hex).unwrap();
        if !ui::MessageValidator::new(
            &[&"Selector calldata:", &m[0..16],&m[16..32], &m[32..48], &m[48..64]],
            &[&"Confirm"],
            &[&"Cancel"],
        )
        .ask() {
            return Ok(false);
        }   
    }
    Ok(true)
}
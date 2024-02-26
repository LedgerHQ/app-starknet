use ledger_device_sdk::io;
use ledger_device_sdk::ui;

use crate::utils;

pub const WELCOME_SCREEN: &str = "S T A R K N E T";

/// This is the UI flow for signing, composed of a scroller
/// to read the incoming message, a panel that requests user
/// validation, and an exit message.
pub fn sign_ui(message: &[u8]) -> Result<bool, io::SyscallError> {
    ui::gadgets::popup("Tx hash review:");
    {
        let hex: [u8; 64] = utils::to_hex(message).map_err(|_| io::SyscallError::Overflow)?;
        let m = core::str::from_utf8(&hex).map_err(|_| io::SyscallError::InvalidParameter)?;
        ui::gadgets::MessageScroller::new(m).event_loop();
    }

    match ui::gadgets::Validator::new("Sign?").ask() {
        true => Ok(true),
        false => Ok(false),
    }
}

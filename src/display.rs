use ledger_device_sdk::io;
use ledger_device_sdk::ui::*;

use crate::utils;

pub const WELCOME_SCREEN: &str = "S T A R K N E T";

use core::fmt::{Error, Write};

struct Buf {
    buf: [u8; 64],
    len: usize,
}

impl Write for Buf {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        let bytes = s.as_bytes();

        for (i, b) in bytes.iter().enumerate() {
            self.buf[(self.len + i) % self.buf.len()] = *b;
        }
        self.len += bytes.len();

        Ok(())
    }
}

/// This is the UI flow for signing, composed of a scroller
/// to read the incoming message, a panel that requests user
/// validation, and an exit message.
pub fn sign_ui(message: &[u8]) -> Result<bool, io::SyscallError> {
    gadgets::popup("Tx hash review:");
    {
        let hex: [u8; 64] = utils::to_hex(message).map_err(|_| io::SyscallError::Overflow)?;
        let m = core::str::from_utf8(&hex).map_err(|_| io::SyscallError::InvalidParameter)?;
        gadgets::MessageScroller::new(m).event_loop();
    }

    match gadgets::Validator::new("Sign?").ask() {
        true => Ok(true),
        false => Ok(false),
    }
}

pub fn pkey_ui(key: &[u8]) -> bool {
    let mut b = Buf {
        buf: [0u8; 64],
        len: 0,
    };

    for v in &key[1..33] {
        write!(&mut b, "{:02X}", *v);
    }
    let m = core::str::from_utf8(&b.buf[..b.len]).unwrap();

    let my_field = [gadgets::Field {
        name: "Public Key",
        value: m,
    }];

    let my_review = gadgets::MultiFieldReview::new(
        &my_field,
        &["Confirm Public Key"],
        Some(&bitmaps::EYE),
        "Approve",
        Some(&bitmaps::VALIDATE_14),
        "Reject",
        Some(&bitmaps::CROSSMARK),
    );

    my_review.show()
}

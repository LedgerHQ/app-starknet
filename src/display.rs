use ledger_device_sdk::io;
use ledger_device_sdk::ui::*;

use crate::utils;

pub const WELCOME_SCREEN: &str = "S T A R K N E T";

use core::fmt::{Error, Write};
use core::ops::Shr;

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
pub fn sign_ui(message: &[u8]) -> bool {
    let mut b = Buf {
        buf: [0u8; 64],
        len: 0,
    };

    /* Display 252-bit length Pedersen Hash */
    for v in &message[..31] {
        write!(&mut b, "{:X}", (*v).shr(4));
        write!(&mut b, "{:X}", *v & 15u8);
    }
    write!(&mut b, "{:X}", message[31].shr(4));

    let hash = core::str::from_utf8(&b.buf[..b.len]).unwrap();

    let my_field = [gadgets::Field {
        name: "Hash",
        value: hash,
    }];

    let my_review = gadgets::MultiFieldReview::new(
        &my_field,
        &["Confirm Hash to sign"],
        Some(&bitmaps::EYE),
        "Approve",
        Some(&bitmaps::VALIDATE_14),
        "Reject",
        Some(&bitmaps::CROSSMARK),
    );

    my_review.show()
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

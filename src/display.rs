use include_gif::include_gif;
use ledger_device_sdk::io::{Comm, Event};
use ledger_device_sdk::ui::bitmaps::{
    Glyph, BACK, CERTIFICATE, CROSSMARK, DASHBOARD_X, EYE, VALIDATE_14,
};
use ledger_device_sdk::ui::gadgets::{
    EventOrPageIndex, Field, MultiFieldReview, MultiPageMenu, Page,
};

pub const WELCOME_SCREEN: &str = "Starknet";

use core::fmt::{Error, Write};
use core::ops::Shr;

use crate::Ins;

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
        let _ = write!(&mut b, "{:X}", (*v).shr(4));
        let _ = write!(&mut b, "{:X}", *v & 15u8);
    }
    let _ = write!(&mut b, "{:X}", message[31].shr(4));

    let hash = core::str::from_utf8(&b.buf[..b.len]).unwrap();

    let my_field = [Field {
        name: "Hash",
        value: hash,
    }];

    let my_review = MultiFieldReview::new(
        &my_field,
        &["Confirm Hash to sign"],
        Some(&EYE),
        "Approve",
        Some(&VALIDATE_14),
        "Reject",
        Some(&CROSSMARK),
    );

    my_review.show()
}

pub fn pkey_ui(key: &[u8]) -> bool {
    let mut b = Buf {
        buf: [0u8; 64],
        len: 0,
    };

    for v in &key[1..33] {
        let _ = write!(&mut b, "{:02X}", *v);
    }
    let m = core::str::from_utf8(&b.buf[..b.len]).unwrap();

    let my_field = [Field {
        name: "Public Key",
        value: m,
    }];

    let my_review = MultiFieldReview::new(
        &my_field,
        &["Confirm Public Key"],
        Some(&EYE),
        "Approve",
        Some(&VALIDATE_14),
        "Reject",
        Some(&CROSSMARK),
    );

    my_review.show()
}

fn about_ui(comm: &mut Comm) -> Event<Ins> {
    let pages = [
        &Page::from((["Starknet", "(c) 2024 Ledger"], true)),
        &Page::from(("Back", &BACK)),
    ];
    loop {
        match MultiPageMenu::new(comm, &pages).show() {
            EventOrPageIndex::Event(e) => return e,
            EventOrPageIndex::Index(1) => return main_ui(comm),
            EventOrPageIndex::Index(_) => (),
        }
    }
}

pub fn main_ui(comm: &mut Comm) -> Event<Ins> {
    const APP_ICON: Glyph = Glyph::from_include(include_gif!("starknet_small.gif"));
    let pages = [
        // The from trait allows to create different styles of pages
        // without having to use the new() function.
        &Page::from(([WELCOME_SCREEN, "is ready"], &APP_ICON)),
        &Page::from((["Version", env!("CARGO_PKG_VERSION")], true)),
        &Page::from(("About", &CERTIFICATE)),
        &Page::from(("Quit", &DASHBOARD_X)),
    ];
    loop {
        match MultiPageMenu::new(comm, &pages).show() {
            EventOrPageIndex::Event(e) => return e,
            EventOrPageIndex::Index(2) => return about_ui(comm),
            EventOrPageIndex::Index(3) => ledger_device_sdk::exit_app(0),
            EventOrPageIndex::Index(_) => (),
        }
    }
}

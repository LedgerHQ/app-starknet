use include_gif::include_gif;
use ledger_device_sdk::io::{Comm, Event};

#[cfg(not(any(target_os = "stax", target_os = "flex")))]
use ledger_device_sdk::ui::{
    bitmaps::{Glyph, BACK, CERTIFICATE, CROSSMARK, DASHBOARD_X, EYE, VALIDATE_14},
    gadgets::{EventOrPageIndex, Field, MultiFieldReview, MultiPageMenu, Page},
};

#[cfg(any(target_os = "stax", target_os = "flex"))]
use ledger_device_sdk::nbgl::{NbglAddressReview, NbglGlyph, NbglHomeAndSettings};

use crate::Ins;

/// This is the UI flow for signing, composed of a scroller
/// to read the incoming message, a panel that requests user
/// validation, and an exit message.
pub fn sign_ui(message: &[u8]) -> bool {
    let mut hash_hex = [0u8; 64];
    hex::encode_to_slice(&message[0..32], &mut hash_hex[0..]).unwrap();
    let hash = core::str::from_utf8_mut(&mut hash_hex[..63]).unwrap();
    hash.make_ascii_uppercase();

    #[cfg(not(any(target_os = "stax", target_os = "flex")))]
    {
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

    #[cfg(any(target_os = "stax", target_os = "flex"))]
    {
        // Load glyph from 64x64 4bpp gif file with include_gif macro. Creates an NBGL compatible glyph.
        const APP_ICON: NbglGlyph =
            NbglGlyph::from_include(include_gif!("starknet_64x64.gif", NBGL));
        // Display the address confirmation screen.
        NbglAddressReview::new().glyph(&APP_ICON).show(hash)
    }
}

pub fn pkey_ui(key: &[u8]) -> bool {
    let mut pk_hex = [0u8; 64];
    hex::encode_to_slice(&key[1..33], &mut pk_hex[0..]).unwrap();
    let m = core::str::from_utf8_mut(&mut pk_hex).unwrap();
    m[0..].make_ascii_uppercase();

    #[cfg(not(any(target_os = "stax", target_os = "flex")))]
    {
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
    #[cfg(any(target_os = "stax", target_os = "flex"))]
    {
        // Load glyph from 64x64 4bpp gif file with include_gif macro. Creates an NBGL compatible glyph.
        const APP_ICON: NbglGlyph =
            NbglGlyph::from_include(include_gif!("starknet_64x64.gif", NBGL));
        NbglAddressReview::new().glyph(&APP_ICON).show(m)
    }
}

#[cfg(not(any(target_os = "stax", target_os = "flex")))]
fn about_ui(comm: &mut Comm) -> Event<Ins> {
    #[cfg(not(any(target_os = "stax", target_os = "flex")))]
    {
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
}

#[cfg(not(any(target_os = "stax", target_os = "flex")))]
pub fn main_ui(comm: &mut Comm) -> Event<Ins> {
    const APP_ICON: Glyph = Glyph::from_include(include_gif!("starknet_small.gif"));
    let pages = [
        // The from trait allows to create different styles of pages
        // without having to use the new() function.
        &Page::from((["Starknet", "is ready"], &APP_ICON)),
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

#[cfg(any(target_os = "stax", target_os = "flex"))]
pub fn main_ui(_comm: &mut Comm) -> Event<Ins> {
    // Load glyph from 64x64 4bpp gif file with include_gif macro. Creates an NBGL compatible glyph.
    const APP_ICON: NbglGlyph = NbglGlyph::from_include(include_gif!("starknet_64x64.gif", NBGL));

    // Display the home screen.
    NbglHomeAndSettings::new()
        .glyph(&APP_ICON)
        .infos(
            "Starknet",
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_AUTHORS"),
        )
        .show()
}

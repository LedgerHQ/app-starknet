use include_gif::include_gif;
use ledger_device_sdk::ui::bitmaps::{
    Glyph, BACK, CERTIFICATE, CROSSMARK, DASHBOARD_X, EYE, VALIDATE_14,
};
use ledger_device_sdk::ui::gadgets::{Field, MultiFieldReview, Page};

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

use ledger_device_sdk::ui::gadgets::PageStyle;

pub const APP_ICON: Glyph = Glyph::from_include(include_gif!("starknet_small.gif"));
pub const WELCOME_PAGE: Page = Page::new(
    PageStyle::PictureNormal,
    ["Starknet", "is ready"],
    Some(&APP_ICON),
);
pub const VERSION_PAGE: Page = Page::new(
    PageStyle::BoldNormal,
    ["Version", env!("CARGO_PKG_VERSION")],
    None,
);
pub const ABOUT_PAGE: Page = Page::new(PageStyle::PictureBold, ["About", ""], Some(&CERTIFICATE));
pub const QUIT_PAGE: Page = Page::new(PageStyle::PictureBold, ["Quit", ""], Some(&DASHBOARD_X));
pub const INFO_PAGE: Page = Page::new(PageStyle::BoldNormal, ["Starknet", "(c) 2024 Ledger"], None);
pub const BACK_PAGE: Page = Page::new(PageStyle::PictureBold, ["Back", ""], Some(&BACK));

pub struct PageLink<'a> {
    pub page: &'a Page<'a>,
    pub link: Option<&'a Menu<'a>>,
}

pub static WELCOME_PL: PageLink = PageLink {
    page: &WELCOME_PAGE,
    link: None,
};
pub static VERSION_PL: PageLink = PageLink {
    page: &VERSION_PAGE,
    link: None,
};

pub static ABOUT_PL: PageLink = PageLink {
    page: &ABOUT_PAGE,
    link: Some(&ABOUT_MENU),
};
pub static QUIT_PL: PageLink = PageLink {
    page: &QUIT_PAGE,
    link: Some(&HELL_MENU),
};

pub static INFO_PL: PageLink = PageLink {
    page: &INFO_PAGE,
    link: None,
};

pub static BACK_PL: PageLink = PageLink {
    page: &BACK_PAGE,
    link: Some(&HOME_MENU),
};

pub struct Menu<'a> {
    pub pagelinks: &'a [&'a PageLink<'a>],
}

pub static HELL_MENU: Menu = Menu { pagelinks: &[] };

pub static HOME_MENU: Menu = Menu {
    pagelinks: &[&WELCOME_PL, &VERSION_PL, &ABOUT_PL, &QUIT_PL],
};

pub static ABOUT_MENU: Menu = Menu {
    pagelinks: &[&INFO_PL, &BACK_PL],
};

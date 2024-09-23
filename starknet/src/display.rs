extern crate alloc;
use crate::{
    erc20::{ERC20_TOKENS, TRANSFER},
    types::FieldElement,
};
use alloc::format;
use include_gif::include_gif;
use ledger_device_sdk::{io::Comm, testing};

use crate::context::{Ctx, Transaction};

#[cfg(not(any(target_os = "stax", target_os = "flex")))]
use ledger_device_sdk::ui::{
    bitmaps::{Glyph, BACK, CERTIFICATE, CROSSMARK, DASHBOARD_X, EYE, VALIDATE_14, WARNING},
    gadgets::{
        clear_screen, EventOrPageIndex, Field, MultiFieldReview, MultiPageMenu, Page, PageStyle,
    },
};

#[cfg(any(target_os = "stax", target_os = "flex"))]
use ledger_device_sdk::nbgl::{
    Field, NbglGenericReview, NbglGlyph, NbglHomeAndSettings, NbglPageContent, NbglReview,
    NbglReviewStatus, NbglStatus, TagValueConfirm, TagValueList, TransactionType, TuneIndex,
};

pub fn show_tx(ctx: &mut Ctx) -> Option<bool> {
    match support_clear_sign(&ctx.tx) {
        Some(t) => {
            testing::debug_print("Clear sign supported !!! \n");
            let tx = &ctx.tx;
            let call = &tx.calls[0];

            let sender = tx.sender_address.to_hex_string();
            let token = ERC20_TOKENS[t].ticker;
            let to = call.calldata[0].to_hex_string();
            let amount = call.calldata[1].to_dec_string(Some(ERC20_TOKENS[t].decimals));

            testing::debug_print("Compute fees \n");

            let max_fees_str = match tx.version {
                FieldElement::ONE => {
                    let mut max_fees_str = tx.max_fee.to_dec_string(None);
                    max_fees_str.push_str(" wei");
                    max_fees_str
                }
                FieldElement::THREE => {
                    let max_amount = FieldElement::from(&tx.l1_gas_bounds.value[8..16]);
                    let max_price_per_unit = FieldElement::from(&tx.l1_gas_bounds.value[16..32]);
                    let max_fees = max_amount * max_price_per_unit;
                    let mut max_fees_str = max_fees.to_dec_string(None);
                    max_fees_str.push_str(" fri");
                    max_fees_str
                }
                _ => {
                    let mut max_fees_str = FieldElement::ZERO.to_dec_string(None);
                    max_fees_str.push_str(" wei");
                    max_fees_str
                }
            };

            testing::debug_print("Compute fees OK \n");

            let my_fields = [
                Field {
                    name: "From",
                    value: sender.as_str(),
                },
                Field {
                    name: "Token",
                    value: token,
                },
                Field {
                    name: "Amount",
                    value: amount.as_str(),
                },
                Field {
                    name: "To",
                    value: to.as_str(),
                },
                Field {
                    name: "Max Fees",
                    value: max_fees_str.as_str(),
                },
            ];

            testing::debug_print(&format!(
                "Token: {}\nTo: {}\nAmount: {}\n",
                token, to, amount
            ));

            #[cfg(not(any(target_os = "stax", target_os = "flex")))]
            {
                let my_review = MultiFieldReview::new(
                    &my_fields,
                    &["Confirm Tx to sign"],
                    Some(&EYE),
                    "Approve",
                    Some(&VALIDATE_14),
                    "Reject",
                    Some(&CROSSMARK),
                );
                Some(my_review.show())
            }
            #[cfg(any(target_os = "stax", target_os = "flex"))]
            {
                // Load glyph from 64x64 4bpp gif file with include_gif macro. Creates an NBGL compatible glyph.
                const APP_ICON: NbglGlyph =
                    NbglGlyph::from_include(include_gif!("starknet_64x64.gif", NBGL));

                let mut review = NbglReview::new()
                    .tx_type(TransactionType::Transaction)
                    .titles("Review", "Transaction", "Sign Transaction")
                    .glyph(&APP_ICON);

                Some(review.show(&my_fields))
            }
        }
        None => {
            testing::debug_print("Clear sign not supported !!! \n");
            None
        }
    }
}

pub fn show_hash(ctx: &mut Ctx, is_tx_hash: bool) -> bool {
    let mut hash = ctx.hash.m_hash.to_hex_string();
    hash.make_ascii_uppercase();

    let my_field = [Field {
        name: match is_tx_hash {
            true => "Transaction Hash",
            false => "Hash",
        },
        value: hash.as_str(),
    }];

    #[cfg(not(any(target_os = "stax", target_os = "flex")))]
    {
        let accept = if is_tx_hash {
            let page_0 = Page::new(
                PageStyle::PictureNormal,
                ["This transaction ", "cannot be trusted"],
                Some(&WARNING),
            );
            let page_1 = Page::new(
                PageStyle::PictureNormal,
                ["Your Ledger cannot ", "decode this transaction."],
                Some(&WARNING),
            );
            let page_2 = Page::new(
                PageStyle::PictureNormal,
                ["If you sign it, you", "could be authorizing"],
                Some(&WARNING),
            );
            let page_3 = Page::new(
                PageStyle::PictureNormal,
                ["malicious actions that", "can drain your wallet."],
                Some(&WARNING),
            );

            clear_screen();
            page_0.place_and_wait();
            clear_screen();
            page_1.place_and_wait();
            clear_screen();
            page_2.place_and_wait();
            clear_screen();
            page_3.place_and_wait();

            let warning_accept = MultiFieldReview::new(
                &[],
                &["I understand the risk"],
                Some(&EYE),
                "Accept",
                Some(&VALIDATE_14),
                "Reject",
                Some(&CROSSMARK),
            );

            warning_accept.show()
        } else {
            true
        };

        match accept {
            false => false,
            true => {
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
        }
    }
    #[cfg(any(target_os = "stax", target_os = "flex"))]
    {
        // Load glyph from 64x64 4bpp gif file with include_gif macro. Creates an NBGL compatible glyph.
        const APP_ICON: NbglGlyph =
            NbglGlyph::from_include(include_gif!("starknet_64x64.gif", NBGL));

        let mut review = NbglReview::new().glyph(&APP_ICON);

        if is_tx_hash {
            review = review
                .tx_type(TransactionType::Transaction)
                .titles("Review", "Transaction", "Sign Transaction")
                .blind();
        } else {
            review = review
                .tx_type(TransactionType::Message)
                .titles("Review", "Hash", "Sign Hash")
                .blind();
        }

        let res = review.show(&my_field);
        ctx.home.show_and_return();
        res
    }
}

pub fn show_pending() {
    #[cfg(not(any(target_os = "stax", target_os = "flex")))]
    {
        let page_0 = Page::new(
            PageStyle::BoldNormal,
            ["Processing ", "Transaction..."],
            None,
        );
        clear_screen();
        page_0.place();
    }
    #[cfg(any(target_os = "stax", target_os = "flex"))]
    {
        let spinner = NbglStatus::new();
        spinner.text("Processing Transaction...").show(true);
    }
}

#[allow(unused_variables)]
pub fn show_status(flag: bool, ctx: &mut Ctx) {
    #[cfg(not(any(target_os = "stax", target_os = "flex")))]
    {
        let content = match flag {
            true => ["Transaction ", "signed"],
            false => ["Transaction ", "rejected"],
        };
        let page_0 = Page::new(PageStyle::BoldNormal, content, None);
        clear_screen();
        page_0.place();
    }
    #[cfg(any(target_os = "stax", target_os = "flex"))]
    {
        let mut status = NbglReviewStatus::new();
        status.show(flag);
        ctx.home.show_and_return();
    }
}

#[allow(unused_variables)]
pub fn pkey_ui(key: &[u8], ctx: &mut Ctx) -> bool {
    let mut pk_hex = [0u8; 64];
    hex::encode_to_slice(&key[1..33], &mut pk_hex[0..]).unwrap();
    let m = core::str::from_utf8_mut(&mut pk_hex).unwrap();
    m[0..].make_ascii_uppercase();

    let my_field = [Field {
        name: "Public Key",
        value: m,
    }];

    #[cfg(not(any(target_os = "stax", target_os = "flex")))]
    {
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
        let tvl = TagValueList::new(&my_field, 4, false, true);
        let tvc = TagValueConfirm::new(&tvl, TuneIndex::LookAtMe, "Approve", "");

        match NbglGenericReview::new()
            .add_content(NbglPageContent::TagValueConfirm(tvc))
            .show("Reject")
        {
            true => {
                let status = NbglStatus::new();
                status.text("Public Key Confirmed").show(true);
                ctx.home.show_and_return();
                true
            }
            false => {
                let status = NbglStatus::new();
                status.text("Public Key Rejected").show(false);
                ctx.home.show_and_return();
                false
            }
        }
    }
}

#[cfg(not(any(target_os = "stax", target_os = "flex")))]
use crate::Ins;
#[cfg(not(any(target_os = "stax", target_os = "flex")))]
use ledger_device_sdk::io::Event;

#[cfg(not(any(target_os = "stax", target_os = "flex")))]
fn about_ui(comm: &mut Comm) -> Event<Ins> {
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
pub fn main_ui_nbgl(_comm: &mut Comm) -> NbglHomeAndSettings {
    // Load glyph from 64x64 4bpp gif file with include_gif macro. Creates an NBGL compatible glyph.
    const APP_ICON: NbglGlyph = NbglGlyph::from_include(include_gif!("starknet_64x64.gif", NBGL));

    // Display the home screen.
    let home = NbglHomeAndSettings::new().glyph(&APP_ICON).infos(
        "Starknet",
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );
    home
}

fn support_clear_sign(tx: &Transaction) -> Option<usize> {
    match tx.calls.len() {
        1 => {
            for (idx, t) in ERC20_TOKENS.iter().enumerate() {
                if tx.calls[0].to == FieldElement::from(t.address)
                    && tx.calls[0].selector == FieldElement::from(TRANSFER)
                {
                    return Some(idx);
                }
            }
            None
        }
        _ => None,
    }
}

extern crate alloc;
use crate::{
    context::{
        Call, DeployAccountTransactionV1, DeployAccountTransactionV3, InvokeTransactionV1,
        InvokeTransactionV3,
    },
    erc20::{ERC20_TOKENS, TRANSFER},
    types::FieldElement,
};

use include_gif::include_gif;
use ledger_device_sdk::io::Comm;

use crate::context::{Ctx, Transaction};

#[cfg(not(any(target_os = "stax", target_os = "flex")))]
use ledger_device_sdk::ui::{
    bitmaps::{Glyph, BACK, CERTIFICATE, CROSSMARK, DASHBOARD_X, EYE, VALIDATE_14, WARNING},
    gadgets::{
        clear_screen, EventOrPageIndex, Field, MultiFieldReview, MultiPageMenu, Page, PageStyle,
    },
};

#[cfg(not(any(target_os = "stax", target_os = "flex")))]
use crate::Ins;

use crate::settings::Settings;
#[cfg(any(target_os = "stax", target_os = "flex"))]
use ledger_device_sdk::nbgl::{
    Field, NbglChoice, NbglGenericReview, NbglGlyph, NbglHomeAndSettings, NbglPageContent,
    NbglReview, NbglReviewStatus, NbglStatus, PageIndex, StatusType, TagValueConfirm, TagValueList,
    TransactionType, TuneIndex,
};

pub fn show_tx(ctx: &mut Ctx) -> Option<bool> {
    let tx = &mut ctx.tx;
    match tx {
        Transaction::None => None,
        Transaction::DeployAccountV3(tx) => show_tx_deploy_account_v3(tx),
        Transaction::DeployAccountV1(tx) => show_tx_deploy_account_v1(tx),
        Transaction::InvokeV3(tx) => show_tx_invoke_v3(tx),
        Transaction::InvokeV1(tx) => show_tx_invoke_v1(tx),
    }
}

fn show_tx_invoke_v3(tx: &InvokeTransactionV3) -> Option<bool> {
    match support_clear_sign(&tx.calls) {
        Some(idx) => {
            let call = &tx.calls[0];

            let mut sender = tx.sender_address.to_hex_string();
            sender.insert_str(0, "0x");
            let token = ERC20_TOKENS[idx].ticker;
            let mut to = call.calldata[0].to_hex_string();
            to.insert_str(0, "0x");
            let amount = call.calldata[1].to_dec_string(Some(ERC20_TOKENS[idx].decimals));

            let max_amount = FieldElement::from(&tx.l1_gas_bounds.value[8..16]);
            let max_price_per_unit = FieldElement::from(&tx.l1_gas_bounds.value[16..32]);
            let max_fees = max_amount * max_price_per_unit;
            let mut max_fees_str = max_fees.to_dec_string(Some(18));
            max_fees_str.push_str(" STRK");

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

                let review = NbglReview::new()
                    .tx_type(TransactionType::Transaction)
                    .titles("Review transaction", "", "Sign Transaction ?")
                    .glyph(&APP_ICON);

                Some(review.show(&my_fields))
            }
        }
        None => None,
    }
}

fn show_tx_invoke_v1(tx: &InvokeTransactionV1) -> Option<bool> {
    match support_clear_sign(&tx.calls) {
        Some(idx) => {
            let call = &tx.calls[0];

            let mut sender = tx.sender_address.to_hex_string();
            sender.insert_str(0, "0x");
            let token = ERC20_TOKENS[idx].ticker;
            let mut to = call.calldata[0].to_hex_string();
            to.insert_str(0, "0x");
            let amount = call.calldata[1].to_dec_string(Some(ERC20_TOKENS[idx].decimals));

            let mut max_fees_str = tx.max_fee.to_dec_string(Some(18));
            max_fees_str.push_str(" ETH");

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

                let review = NbglReview::new()
                    .tx_type(TransactionType::Transaction)
                    .titles("Review transaction", "", "Sign Transaction ?")
                    .glyph(&APP_ICON);

                Some(review.show(&my_fields))
            }
        }
        None => None,
    }
}

fn show_tx_deploy_account_v3(tx: &DeployAccountTransactionV3) -> Option<bool> {
    let mut contract_address = tx.contract_address.to_hex_string();
    contract_address.insert_str(0, "0x");

    let mut class_hash = tx.class_hash.to_hex_string();
    class_hash.insert_str(0, "0x");

    let max_amount = FieldElement::from(&tx.l1_gas_bounds.value[8..16]);
    let max_price_per_unit = FieldElement::from(&tx.l1_gas_bounds.value[16..32]);
    let max_fees = max_amount * max_price_per_unit;
    let mut fees = max_fees.to_dec_string(Some(18));
    fees.push_str(" STRK");

    let my_fields = [
        Field {
            name: "Deploy account",
            value: contract_address.as_str(),
        },
        Field {
            name: "Max Fees",
            value: fees.as_str(),
        },
        Field {
            name: "Class Hash",
            value: class_hash.as_str(),
        },
    ];

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

        let review = NbglReview::new()
            .tx_type(TransactionType::Transaction)
            .titles("Review transaction", "", "Sign Transaction ?")
            .glyph(&APP_ICON);

        Some(review.show(&my_fields))
    }
}

fn show_tx_deploy_account_v1(tx: &DeployAccountTransactionV1) -> Option<bool> {
    // display contract_address, fees and class_hash
    let mut contract_address = tx.contract_address.to_hex_string();
    contract_address.insert_str(0, "0x");

    let mut class_hash = tx.class_hash.to_hex_string();
    class_hash.insert_str(0, "0x");

    let mut fees = tx.max_fee.to_dec_string(Some(18));
    fees.push_str(" ETH");

    let my_fields = [
        Field {
            name: "Deploy account",
            value: contract_address.as_str(),
        },
        Field {
            name: "Max Fees",
            value: fees.as_str(),
        },
        Field {
            name: "Class Hash",
            value: class_hash.as_str(),
        },
    ];

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

        let review = NbglReview::new()
            .tx_type(TransactionType::Transaction)
            .titles("Review transaction", "", "Sign Transaction ?")
            .glyph(&APP_ICON);

        Some(review.show(&my_fields))
    }
}

pub fn show_hash(ctx: &mut Ctx, is_tx_hash: bool) -> bool {
    let mut hash = ctx.hash.to_hex_string();
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
        let page = Page::new(PageStyle::PictureBold, ["Blind", "Signing"], Some(&WARNING));
        clear_screen();
        page.place_and_wait();

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

        let mut review = NbglReview::new().glyph(&APP_ICON);

        if is_tx_hash {
            review = review
                .tx_type(TransactionType::Transaction)
                .titles("Review transaction", "", "Sign Transaction ?")
                .blind();
        } else {
            review = review
                .tx_type(TransactionType::Message)
                .titles("Review hash", "", "Sign Hash ?")
                .blind();
        }

        review.show(&my_field)
    }
}

pub fn show_step(text: &str, ctx: &mut Ctx) {
    #[cfg(any(target_os = "nanosplus", target_os = "nanox"))]
    {
        let _c = ctx;
        let (s1, s2) = text.split_once(' ').unwrap();

        let page_0 = Page::new(PageStyle::BoldNormal, [s1, s2], None);
        clear_screen();
        page_0.place();
    }

    #[cfg(any(target_os = "stax", target_os = "flex"))]
    {
        ctx.spinner.show(text);
    }
}

#[allow(unused_variables)]
pub fn show_status(flag: bool, is_tx: bool, ctx: &mut Ctx) {
    #[cfg(not(any(target_os = "stax", target_os = "flex")))]
    {
        let msg = match is_tx {
            true => "Transaction ",
            false => "Message ",
        };
        let content = match flag {
            true => [msg, "signed"],
            false => [msg, "rejected"],
        };
        let page_0 = Page::new(PageStyle::BoldNormal, content, None);
        clear_screen();
        page_0.place();
    }
    #[cfg(any(target_os = "stax", target_os = "flex"))]
    {
        let status = match is_tx {
            true => NbglReviewStatus::new().status_type(StatusType::Transaction),
            false => NbglReviewStatus::new().status_type(StatusType::Message),
        };
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
fn settings_ui(comm: &mut Comm) -> Event<Ins> {
    {
        let settings: Settings = Default::default();
        let mut bs_enabled: bool = settings.get_element(0) != 0;
        let mut bs_status = if bs_enabled { "Enabled" } else { "Disabled" };

        loop {
            let pages = [
                &Page::from((["Blind Signing", bs_status], true)),
                &Page::from(("Back", &BACK)),
            ];
            match MultiPageMenu::new(comm, &pages).show() {
                EventOrPageIndex::Event(e) => return e,
                EventOrPageIndex::Index(0) => {
                    bs_enabled = !bs_enabled;
                    match bs_enabled {
                        true => {
                            settings.set_element(0, 1);
                            bs_status = "Enabled";
                        }
                        false => {
                            settings.set_element(0, 0);
                            bs_status = "Disabled";
                        }
                    }
                }
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
        &Page::from(("Settings", &EYE)),
        &Page::from(("About", &CERTIFICATE)),
        &Page::from(("Quit", &DASHBOARD_X)),
    ];
    loop {
        match MultiPageMenu::new(comm, &pages).show() {
            EventOrPageIndex::Event(e) => return e,
            EventOrPageIndex::Index(2) => return settings_ui(comm),
            EventOrPageIndex::Index(3) => return about_ui(comm),
            EventOrPageIndex::Index(4) => ledger_device_sdk::exit_app(0),
            EventOrPageIndex::Index(_) => (),
        }
    }
}

#[cfg(any(target_os = "stax", target_os = "flex"))]
pub fn main_ui_nbgl(_comm: &mut Comm) -> NbglHomeAndSettings {
    // Load glyph from 64x64 4bpp gif file with include_gif macro. Creates an NBGL compatible glyph.
    const APP_ICON: NbglGlyph = NbglGlyph::from_include(include_gif!("starknet_64x64.gif", NBGL));

    let settings_strings = [["Blind signing", "Enable transaction blind signing"]];
    let mut settings: Settings = Default::default();

    // Display the home screen.
    NbglHomeAndSettings::new()
        .glyph(&APP_ICON)
        .infos(
            "Starknet",
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_AUTHORS"),
        )
        .settings(settings.get_mut(), &settings_strings)
}

#[allow(unused_variables)]
pub fn blind_signing_enable_ui(ctx: &mut Ctx) {
    #[cfg(not(any(target_os = "stax", target_os = "flex")))]
    {
        let page = Page::new(
            PageStyle::PictureNormal,
            ["Blind signing must ", "be enabled in Settings"],
            Some(&CROSSMARK),
        );

        clear_screen();
        page.place_and_wait();
    }
    #[cfg(any(target_os = "stax", target_os = "flex"))]
    {
        let choice = NbglChoice::new().show(
            "This transaction cannot be clear-signed",
            "Enable blind-signing in the settings to sign this transaction",
            "Go to settings",
            "Reject transaction",
        );
        if choice {
            ctx.home.set_start_page(PageIndex::Settings(0));
            ctx.home.show_and_return();
            ctx.home.set_start_page(PageIndex::Home);
        } else {
            ctx.home.show_and_return();
        }
    }
}

fn support_clear_sign(calls: &[Call]) -> Option<usize> {
    match calls.len() {
        1 => {
            for (idx, t) in ERC20_TOKENS.iter().enumerate() {
                if calls[0].to == FieldElement::from(t.address)
                    && calls[0].selector == FieldElement::from(TRANSFER)
                {
                    return Some(idx);
                }
            }
            None
        }
        _ => None,
    }
}

extern crate alloc;
use crate::{
    context::{
        Call, DeployAccountTransactionV1, DeployAccountTransactionV3, InvokeTransactionV1,
        InvokeTransactionV3,
    },
    erc20::{ERC20_TOKENS, TRANSFER},
    transaction,
    types::FieldElement,
};
use alloc::{format, string::String, vec::Vec};
use num_bigint::BigUint;

use ledger_device_sdk::include_gif;
use ledger_device_sdk::io::Comm;

use crate::context::{Ctx, Transaction};

use crate::settings::Settings;
use ledger_device_sdk::nbgl::{
    Field, NbglChoice, NbglGenericReview, NbglGlyph, NbglHomeAndSettings, NbglPageContent,
    NbglReview, NbglReviewStatus, NbglStatus, PageIndex, StatusType, TagValueConfirm, TagValueList,
    TransactionType, TuneIndex,
};

/// Status word for a transaction the app refuses outright, as opposed to one
/// it merely cannot display and routes to blind signing.
/// 0xFF03, 0xFF04, 0xFF06 and 0xFF07 are taken by transaction::ParseError.
#[derive(Clone, Copy)]
pub enum TxError {
    MalformedTransferCalldata = 0xFF02,
    /// The host's Deploy Account address is not the one the signed class
    /// hash, salt and constructor calldata deploy to.
    ContractAddressMismatch = 0xFF05,
}

fn hex(fe: &FieldElement) -> String {
    let mut s = fe.to_hex_string();
    s.insert_str(0, "0x");
    s
}

fn max_amount(bound: &FieldElement) -> BigUint {
    BigUint::from_bytes_be(&bound.value[8..16])
}

/// A FRI amount as STRK. Everything computed from V3 bounds and tip stays
/// below 3 * 2^192 + 2^128, well within a felt.
fn strk(fri: BigUint) -> String {
    let mut s = FieldElement::from(fri.to_bytes_be().as_slice()).to_dec_string(Some(18));
    s.push_str(" STRK");
    s
}

/// The most the tip can add to the fee. The tip is a price per unit of L2
/// gas, paid on every unit the transaction may use.
fn max_tip_v3(tip: &FieldElement, l2_gas_bounds: &FieldElement) -> BigUint {
    BigUint::from_bytes_be(&tip.value) * max_amount(l2_gas_bounds)
}

/// Worst-case STRK debit of a V3 transaction, in integer arithmetic: each
/// resource's max_amount * max_price_per_unit, plus the maximum tip. The tip
/// is signed along with the bounds, so it counts towards what the user
/// approves; adding it on top bounds the charge whether or not the network
/// takes it within max_price_per_unit.
fn max_fees_v3(tip: &FieldElement, bounds: [&FieldElement; 3]) -> String {
    let price = |b: &FieldElement| BigUint::from_bytes_be(&b.value[16..32]);

    // bounds[1] is L2 gas.
    let mut total = max_tip_v3(tip, bounds[1]);
    for b in bounds {
        total += max_amount(b) * price(b);
    }
    strk(total)
}

/// The tip gets a line of its own when there is one, as the most it can add
/// to the fee. It is shown whenever it is non-zero, even when no L2 gas is
/// allowed and it cannot cost anything, so that a signed tip always shows on
/// screen. Transactions without a tip, which is most of them, review exactly
/// as they did before.
fn tip_v3(tip: &FieldElement, l2_gas_bounds: &FieldElement) -> Option<String> {
    (*tip != FieldElement::ZERO).then(|| strk(max_tip_v3(tip, l2_gas_bounds)))
}

/// What the review UI can do with a call.
enum ClearSign {
    /// Reviewable field by field; index into ERC20_TOKENS.
    Supported(usize),
    /// Not something this app knows how to display. Blind signing may apply.
    Unsupported,
    /// A known ERC20 `transfer` whose calldata cannot be a valid transfer.
    /// Not a candidate for blind signing: it cannot execute on chain, and the
    /// app knows that, so offering it as an opaque hash would misrepresent
    /// what the device actually understood.
    Invalid,
}

pub fn show_tx(ctx: &mut Ctx) -> Result<Option<bool>, TxError> {
    let tx = &mut ctx.tx;
    match tx {
        Transaction::None => Ok(None),
        Transaction::DeployAccountV3(tx) => show_tx_deploy_account_v3(tx),
        Transaction::DeployAccountV1(tx) => show_tx_deploy_account_v1(tx),
        Transaction::InvokeV3(tx) => show_tx_invoke_v3(tx),
        Transaction::InvokeV1(tx) => show_tx_invoke_v1(tx),
    }
}

fn show_tx_invoke_v3(tx: &InvokeTransactionV3) -> Result<Option<bool>, TxError> {
    if tx.nb_calls > 1 {
        return Ok(None);
    }
    match support_clear_sign(&tx.call) {
        ClearSign::Supported(idx) => {
            let call = &tx.call;

            let mut sender = tx.sender_address.to_hex_string();
            sender.insert_str(0, "0x");
            let token = ERC20_TOKENS[idx].ticker;
            let mut to = call.calldata[0].to_hex_string();
            to.insert_str(0, "0x");
            let amount = call.calldata[1].to_dec_string(Some(ERC20_TOKENS[idx].decimals));

            let max_fees_str = max_fees_v3(
                &tx.tip,
                [&tx.l1_gas_bounds, &tx.l2_gas_bounds, &tx.l1_data_gas_bounds],
            );
            let tip = tip_v3(&tx.tip, &tx.l2_gas_bounds);

            let mut my_fields = Vec::from([
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
            ]);
            if let Some(tip) = &tip {
                my_fields.push(Field {
                    name: "Max Tip",
                    value: tip.as_str(),
                });
            }

            // Load glyph from file with include_gif macro. Creates an NBGL compatible glyph.
            #[cfg(any(target_os = "nanosplus", target_os = "nanox"))]
            const APP_ICON: NbglGlyph =
                NbglGlyph::from_include(include_gif!("starknet_small.gif", NBGL));
            #[cfg(any(target_os = "stax", target_os = "flex"))]
            const APP_ICON: NbglGlyph =
                NbglGlyph::from_include(include_gif!("starknet_64x64.gif", NBGL));
            #[cfg(target_os = "apex_p")]
            const APP_ICON: NbglGlyph =
                NbglGlyph::from_include(include_gif!("starknet_48x48.png", NBGL));

            let review = NbglReview::new()
                .tx_type(TransactionType::Transaction)
                .titles("Review transaction", "", "Sign Transaction ?")
                .glyph(&APP_ICON);

            Ok(Some(review.show(&my_fields)))
        }
        ClearSign::Unsupported => Ok(None),
        ClearSign::Invalid => Err(TxError::MalformedTransferCalldata),
    }
}

fn show_tx_invoke_v1(tx: &InvokeTransactionV1) -> Result<Option<bool>, TxError> {
    if tx.nb_calls > 1 {
        return Ok(None);
    }
    match support_clear_sign(&tx.call) {
        ClearSign::Supported(idx) => {
            let call = &tx.call;

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

            // Load glyph from file with include_gif macro. Creates an NBGL compatible glyph.
            #[cfg(any(target_os = "nanosplus", target_os = "nanox"))]
            const APP_ICON: NbglGlyph =
                NbglGlyph::from_include(include_gif!("starknet_small.gif", NBGL));
            #[cfg(any(target_os = "stax", target_os = "flex"))]
            const APP_ICON: NbglGlyph =
                NbglGlyph::from_include(include_gif!("starknet_64x64.gif", NBGL));
            #[cfg(target_os = "apex_p")]
            const APP_ICON: NbglGlyph =
                NbglGlyph::from_include(include_gif!("starknet_48x48.png", NBGL));

            let review = NbglReview::new()
                .tx_type(TransactionType::Transaction)
                .titles("Review transaction", "", "Sign Transaction ?")
                .glyph(&APP_ICON);

            Ok(Some(review.show(&my_fields)))
        }
        ClearSign::Unsupported => Ok(None),
        ClearSign::Invalid => Err(TxError::MalformedTransferCalldata),
    }
}

fn show_tx_deploy_account_v3(tx: &DeployAccountTransactionV3) -> Result<Option<bool>, TxError> {
    let fees = max_fees_v3(
        &tx.tip,
        [&tx.l1_gas_bounds, &tx.l2_gas_bounds, &tx.l1_data_gas_bounds],
    );
    show_deploy_account(
        &tx.contract_address,
        &tx.class_hash,
        &tx.contract_address_salt,
        &tx.constructor_calldata,
        &fees,
        tip_v3(&tx.tip, &tx.l2_gas_bounds),
    )
}

fn show_tx_deploy_account_v1(tx: &DeployAccountTransactionV1) -> Result<Option<bool>, TxError> {
    let mut fees = tx.max_fee.to_dec_string(Some(18));
    fees.push_str(" ETH");
    show_deploy_account(
        &tx.contract_address,
        &tx.class_hash,
        &tx.contract_address_salt,
        &tx.constructor_calldata,
        &fees,
        None,
    )
}

/// The constructor arguments decide who controls the deployed account (owner
/// key, guardian, recovery), and the salt goes into its address, so both are
/// signed and both are shown. The address itself comes from the host: it is
/// only displayed once it matches the one those values actually deploy to.
fn show_deploy_account(
    contract_address: &FieldElement,
    class_hash: &FieldElement,
    salt: &FieldElement,
    constructor_calldata: &[FieldElement],
    fees: &str,
    tip: Option<String>,
) -> Result<Option<bool>, TxError> {
    if transaction::contract_address(*class_hash, *salt, constructor_calldata) != *contract_address
    {
        return Err(TxError::ContractAddressMismatch);
    }

    let contract_address = hex(contract_address);
    let class_hash = hex(class_hash);
    let salt = hex(salt);
    let args: Vec<(String, String)> = constructor_calldata
        .iter()
        .enumerate()
        .map(|(i, d)| (format!("Constructor arg {}", i + 1), hex(d)))
        .collect();

    let mut my_fields = Vec::from([
        Field {
            name: "Deploy account",
            value: contract_address.as_str(),
        },
        Field {
            name: "Max Fees",
            value: fees,
        },
    ]);
    if let Some(tip) = &tip {
        my_fields.push(Field {
            name: "Max Tip",
            value: tip.as_str(),
        });
    }
    my_fields.push(Field {
        name: "Class Hash",
        value: class_hash.as_str(),
    });
    my_fields.push(Field {
        name: "Salt",
        value: salt.as_str(),
    });
    for (name, value) in &args {
        my_fields.push(Field {
            name: name.as_str(),
            value: value.as_str(),
        });
    }

    // Load glyph from file with include_gif macro. Creates an NBGL compatible glyph.
    #[cfg(any(target_os = "nanosplus", target_os = "nanox"))]
    const APP_ICON: NbglGlyph = NbglGlyph::from_include(include_gif!("starknet_small.gif", NBGL));
    #[cfg(any(target_os = "stax", target_os = "flex"))]
    const APP_ICON: NbglGlyph = NbglGlyph::from_include(include_gif!("starknet_64x64.gif", NBGL));
    #[cfg(target_os = "apex_p")]
    const APP_ICON: NbglGlyph = NbglGlyph::from_include(include_gif!("starknet_48x48.png", NBGL));

    let review = NbglReview::new()
        .tx_type(TransactionType::Transaction)
        .titles("Review transaction", "", "Sign Transaction ?")
        .glyph(&APP_ICON);

    Ok(Some(review.show(&my_fields)))
}

pub fn show_hash(ctx: &mut Ctx, is_tx_hash: bool) -> bool {
    let mut hash = ctx.hash.to_hex_string();
    hash.make_ascii_uppercase();

    let my_field = [Field {
        name: match is_tx_hash {
            #[cfg(any(target_os = "stax", target_os = "flex", target_os = "apex_p"))]
            true => "Transaction Hash",
            #[cfg(any(target_os = "nanosplus", target_os = "nanox"))]
            true => "Tx Hash",
            false => "Hash",
        },
        value: hash.as_str(),
    }];

    // Load glyph from file with include_gif macro. Creates an NBGL compatible glyph.
    #[cfg(any(target_os = "nanosplus", target_os = "nanox"))]
    const APP_ICON: NbglGlyph = NbglGlyph::from_include(include_gif!("starknet_small.gif", NBGL));
    #[cfg(any(target_os = "stax", target_os = "flex"))]
    const APP_ICON: NbglGlyph = NbglGlyph::from_include(include_gif!("starknet_64x64.gif", NBGL));
    #[cfg(target_os = "apex_p")]
    const APP_ICON: NbglGlyph = NbglGlyph::from_include(include_gif!("starknet_48x48.png", NBGL));

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

pub fn show_step(text: &str, ctx: &mut Ctx) {
    ctx.spinner.show(text);
}

#[allow(unused_variables)]
pub fn show_status(flag: bool, is_tx: bool, ctx: &mut Ctx) {
    let status = match is_tx {
        true => NbglReviewStatus::new().status_type(StatusType::Transaction),
        false => NbglReviewStatus::new().status_type(StatusType::Message),
    };
    status.show(flag);
    ctx.home.show_and_return();
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

pub fn main_ui_nbgl(_comm: &mut Comm) -> NbglHomeAndSettings {
    // Load glyph from file with include_gif macro. Creates an NBGL compatible glyph.
    #[cfg(any(target_os = "nanosplus", target_os = "nanox"))]
    const APP_ICON: NbglGlyph = NbglGlyph::from_include(include_gif!("home_nano_nbgl.png", NBGL));
    #[cfg(any(target_os = "stax", target_os = "flex"))]
    const APP_ICON: NbglGlyph = NbglGlyph::from_include(include_gif!("starknet_64x64.gif", NBGL));
    #[cfg(target_os = "apex_p")]
    const APP_ICON: NbglGlyph = NbglGlyph::from_include(include_gif!("starknet_48x48.png", NBGL));

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

fn support_clear_sign(call: &Call) -> ClearSign {
    // Identify the call first: only a transfer to a token we know is held to
    // the ERC20 ABI below. Anything else is simply undisplayable, not invalid.
    let mut token = None;
    for (idx, t) in ERC20_TOKENS.iter().enumerate() {
        if call.to == FieldElement::from(t.address) && call.selector == FieldElement::from(TRANSFER)
        {
            token = Some(idx);
            break;
        }
    }
    let Some(idx) = token else {
        return ClearSign::Unsupported;
    };

    // transfer(recipient, amount: u256) serializes to exactly 3 felts. Check
    // both the announced length and what was actually stored: calldata sent in
    // a SetCallStep::Add chunk is counted and hashed but not kept in
    // call.calldata, so the two can differ. Any other length cannot deserialize into the
    // entrypoint on chain, so the transaction is refused rather than signed.
    if call.nb_calldata != 3 || call.calldata.len() != 3 {
        return ClearSign::Invalid;
    }

    // The amount is displayed from the low word only. Any real balance fits in
    // it, so a non-zero high word means the screen would understate the amount.
    // The call is still well formed, so it falls back to blind signing rather
    // than being refused.
    if call.calldata[2] != FieldElement::ZERO {
        return ClearSign::Unsupported;
    }

    ClearSign::Supported(idx)
}
